#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl Color {
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Color {
      red,
      green,
      blue,
      alpha: 0xFF,
    }
  }

  const BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0xFF,
  };
}

impl Default for Color {
  fn default() -> Self { Color::BLACK }
}
