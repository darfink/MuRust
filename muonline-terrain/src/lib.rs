use pathfinding;
use std::io::{Error, ErrorKind, Read, Result};
use super::Position;

pub use self::node::Node;
mod node;

/// A terrain object, acting as an interface to all nodes.
#[derive(Clone, Debug)]
pub struct Terrain {
    width: usize,
    height: usize,
    attrs: Vec<Node>,
}

impl Terrain {
    /// Reads a terrain object from raw bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        const HEADER_C: u8 = 0;
        const HEADER_SIZE: usize = 3;

        validate(
            bytes.len() >= HEADER_SIZE,
            format!("invalid size {}, expected at least {}", bytes.len(), HEADER_SIZE))?;

        let (header, data) = bytes.split_at(HEADER_SIZE);
        let (head, width, height) = (header[0], header[1], header[2]);

        validate(
            head == HEADER_C,
            format!("invalid head ID {}, expected {}", head, HEADER_C))?;

        let width = width as usize;
        let height = height as usize;
        let raw_size = (width + 1) * (height + 1);

        validate(
            data.len() == raw_size,
            format!("invalid content size {}, expected {}", data.len(), raw_size))?;

        let mut attrs = Vec::with_capacity(width * height);
        for y in 0..height {
            let base = y * width + y;
            let slice = data[base..(base + width)].iter();

            attrs.extend(slice.map(|v| Node::from_bits(*v).unwrap()));
        }

        Ok(Terrain { attrs, width, height })
    }

    /// Extracts a terrain object from an input source.
    pub fn from_input(input: &mut Read) -> Result<Self> {
        let mut bytes = Vec::new();
        input.read_to_end(&mut bytes)?;
        Self::from_bytes(&bytes)
    }

    // Returns a list of nodes that represents the most efficient path from start to goal.
    pub fn find_path<P1, P2>(&self, start: P1, goal: P2) -> Option<Vec<Position>>
            where P1: Into<Position>, P2: Into<Position> {
        let goal: Position = goal.into();
        pathfinding::astar(
            &start.into(),
            |pos| {
                // TODO: Use SmallVec (stack allocation) instead
                pos.neighbors(1)
                   .filter_map(|pos| self.get(pos).map(|attr| (pos, attr)))
                   .filter(|&(_, attr)| attr.is_free())
                   .map(|(pos, _)| (pos, 1))
                   .collect::<Vec<(Position, usize)>>()
            },
            |pos| pos.distance(&goal) / 3,
            |pos| pos == &goal)
        .map(|(path, cost)| path)
    }

    /// Returns an iterator over each of the terrain's row.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&[Node]> + 'a {
        self.attrs.chunks(self.width)
    }

    /// Returns a position's node.
    pub fn get<P: Into<Position>>(&self, pos: P) -> Option<&Node> {
        let pos: Position = pos.into();
        if pos.x < self.width() || pos.y < self.height() {
            Some(&self.attrs[pos.y * self.width + pos.x])
        } else {
            None
        }
    }

    /// Returns a position's mutable node.
    pub fn get_mut<P: Into<Position>>(&mut self, pos: P) -> Option<&mut Node> {
        let pos: Position = pos.into();
        if pos.x < self.width() || pos.y < self.height() {
            Some(&mut self.attrs[pos.y * self.width + pos.x])
        } else {
            None
        }
    }

    /// Returns the terrain's width.
    pub fn width(&self) -> usize { self.width }

    /// Returns the terrain's height.
    pub fn height(&self) -> usize { self.height }

    /// Returns the terrain's area.
    pub fn size(&self) -> usize { self.width * self.height }
}

/// Returns an `InvalidData` result from a boolean.
fn validate<S: Into<String>>(result: bool, message: S) -> Result<()> {
    if !result {
        Err(Error::new(ErrorKind::InvalidData, message.into()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TERRAIN: &[u8] = include_bytes!("../test/terrain/Devias.att");

    #[test]
    fn default() {
        let terrain = Terrain::from_bytes(TERRAIN).unwrap();
        let _ = terrain.find_path((242, 10), (30, 25)).unwrap();
    }
}
