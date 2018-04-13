#[derive(Debug)]
pub struct Position {
  pub x: u8,
  pub y: u8,
}

impl Position {
  pub fn new(x: u8, y: u8) -> Self { Position { x, y } }
}
