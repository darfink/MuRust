/// A player's hero status.
#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum HeroStatus {
  Revered = 1,
  Hero = 2,
  Commoner = 3,
  Outcast = 4,
  Outlaw = 5,
  Murderer = 6,
}

impl Default for HeroStatus {
  fn default() -> Self { HeroStatus::Commoner }
}

primitive_serialize!(HeroStatus, u8);
