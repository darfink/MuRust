/// A player's kill status.
#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum PkStatus {
  Revered = 1,
  Hero = 2,
  Commoner = 3,
  Outcast = 4,
  Outlaw = 5,
  Murderer = 6,
}

impl Default for PkStatus {
  fn default() -> Self { PkStatus::Commoner }
}

primitive_serialize!(PkStatus, u8);
