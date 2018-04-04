use muserialize::IntegerLE;

/// An identifier for a Game Server.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GameServerCode {
  #[serde(with = "IntegerLE")]
  code: u16,
}

/// The Game Server IDs' are segmented into 20 number intervals.
const GS_GROUP_MOD: u16 = 20;

impl GameServerCode {
  pub fn new(id: u8, group: u8) -> Self {
    assert!(id > 0 && id < 20);
    GameServerCode {
      code: (group - 1) as u16 * GS_GROUP_MOD + (id as u16 - 1),
    }
  }

  pub fn id(&self) -> u8 { (self.code % GS_GROUP_MOD + 1) as u8 }

  pub fn group(&self) -> u8 { (self.code / GS_GROUP_MOD + 1) as u8 }

  pub fn code(&self) -> u16 { self.code }
}

impl From<u16> for GameServerCode {
  fn from(code: u16) -> Self { GameServerCode { code } }
}
