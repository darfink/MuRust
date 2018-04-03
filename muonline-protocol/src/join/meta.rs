//! Join packets' meta.

use model::{GameServerCode, GameServerLoad};

/// A Game Server list entry.
///
/// Used in conjunction with [GameServerList](../struct.GameServerList.html).
#[derive(Serialize, Debug)]
pub struct GameServerListEntry {
  /// The Game Server's identifier.
  pub code: GameServerCode,
  /// The Game Server's load.
  pub load: GameServerLoad,
  /// Unknown field.
  pub unknown: u8,
}

impl GameServerListEntry {
  pub fn new(code: GameServerCode, load: GameServerLoad) -> Self {
    GameServerListEntry {
      code,
      load,
      unknown: 0x77,
    }
  }
}
