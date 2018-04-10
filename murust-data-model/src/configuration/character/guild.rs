/// A player's guild role.
// TODO: GuildMaster + BattleMaster?
#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum GuildRole {
  Private = 0x00,
  Corporal = 0x20,
  Sergeant = 0x40,
  Lieutenant = 0x80,
  None = 0xFF,
}

impl Default for GuildRole {
  fn default() -> Self { GuildRole::None }
}

primitive_serialize!(GuildRole, u8);
