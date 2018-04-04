//! Contains packet meta attributes.

use join::model::GameServerLoad;
use muserialize::IntegerLE;

/// A Game Server list entry.
///
/// Used in conjunction with [GameServerList](../struct.GameServerList.html).
#[derive(Serialize, Debug)]
pub struct GameServerListEntry {
  /// The Game Server's identifier.
  #[serde(with = "IntegerLE")]
  pub id: u16,
  /// The Game Server's load.
  pub load: GameServerLoad,
  /// Unknown field.
  pub unknown: u8,
}

impl GameServerListEntry {
  pub fn new<T: Into<GameServerLoad>>(id: u16, load: T) -> Self {
    GameServerListEntry {
      id,
      load: load.into(),
      unknown: 0x77,
    }
  }
}
