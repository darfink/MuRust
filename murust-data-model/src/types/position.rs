use serde::{Deserialize, Deserializer, Serialize, Serializer};
use types::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
  pub x: u8,
  pub y: u8,
}

impl Position {
  /// Constructs a new position.
  pub fn new(x: u8, y: u8) -> Self { Position { x, y } }

  /// Advances a position one step in a direction.
  pub fn advance(self, direction: Direction) -> Self {
    match direction {
      Direction::SouthWest => Position::new(self.x.saturating_sub(1), self.y.saturating_sub(1)),
      Direction::South => Position::new(self.x, self.y.saturating_sub(1)),
      Direction::SouthEast => Position::new(self.x.saturating_add(1), self.y.saturating_sub(1)),
      Direction::East => Position::new(self.x.saturating_add(1), self.y),
      Direction::NorthEast => Position::new(self.x.saturating_add(1), self.y.saturating_add(1)),
      Direction::North => Position::new(self.x, self.y.saturating_add(1)),
      Direction::NorthWest => Position::new(self.x.saturating_sub(1), self.y.saturating_add(1)),
      Direction::West => Position::new(self.x.saturating_sub(1), self.y),
    }
  }

  /// Returns the distance between two points.
  pub fn distance(&self, other: &Self) -> u8 {
    ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs()) as u8
  }

  /// Returns a coordinates neighbors within a specified distance.
  pub fn neighbors<'a>(&'a self, distance: u8) -> impl Iterator<Item = Self> + 'a {
    let (start_x, end_x) = (
      self.x.saturating_sub(distance),
      self.x.saturating_add(distance),
    );

    (self.y.saturating_sub(distance)..=self.y.saturating_add(distance))
      .flat_map(move |y| (start_x..=end_x).map(move |x| Position { x, y }))
      .filter(move |&pos| pos != *self)
  }
}

impl From<(u8, u8)> for Position {
  fn from((x, y): (u8, u8)) -> Self { Position::new(x, y) }
}

impl Serialize for Position {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    [self.x, self.y].serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Position {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    <[u8; 2]>::deserialize(deserializer).map(|value| Position::new(value[0], value[1]))
  }
}
