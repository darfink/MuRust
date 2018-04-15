/// A collection of all possible directions.
#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
  Undefined = 0,
  South = 1,
  SouthEast = 2,
  East = 3,
  NorthEast = 4,
  North = 5,
  NorthWest = 6,
  West = 7,
  SouthWest = 8,
}

primitive_serialize!(Direction, u8);

impl Default for Direction {
  fn default() -> Self { Direction::Undefined }
}
