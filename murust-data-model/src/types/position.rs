use types::Direction;

#[derive(Debug, Copy, Clone)]
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
}
