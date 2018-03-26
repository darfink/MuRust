//! Join Server
//!
//! ## Handshake
//!
//! The handshake is initialized by the client, and ensures that the client and
//! server uses a compatible protocol version.
//!
//! ## Example Session
//!
//! > Client → Server ([JoinServerConnectRequest](../client/struct.JoinServerConnectRequest.html))
//!
//! ```c
//! [0xC1, 0x06, 0xA9, 0x00, 0x00, 0x01]
//! ```
//!
//! > Server → Client
//! ([JoinServerConnectResult](./struct.JoinServerConnectResult.html))
//!
//! ```c
//! [0xC1, 0x04, 0x00, 0x01]
//! ```
//!
//! > Client → Server
//! ([GameServerListRequest](../client/struct.GameServerListRequest.html))
//!
//! ```c
//! [0xC1, 0x04, 0xF4, 0x06]
//! ```
//!
//! > Server → Client ([GameServerList](./struct.GameServerList.html))
//!
//! ```c
//! [0xC2, 0x00, 0x0B, 0xF4, 0x06, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00]
//! ```

use muserialize::{IntegerLE, StringFixed, VectorLengthBE};
use std::iter::FromIterator;
use {typenum, GameServerCode, GameServerLoad};

/// `C1:00` — Describes the result of a [JoinServerConnectRequest](../client/struct.JoinServerConnectRequest.html).
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// result | `U8` | Integer representing the result of an operation. | -
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0x00, 0x01]
/// ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "00")]
pub struct JoinServerConnectResult(pub bool);

/// `C1:F4:03` — Contains a Game Server's connection information.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// host | `CHAR(16)` | The game server's IP-address or domain. | -
/// port | `U16` | The game server's active port. | LE
///
/// ## Example
///
/// ```c
/// [0xC1, 0x16, 0xF4, 0x03, 0x38, 0x35, 0x2E, 0x32, 0x32, 0x36, 0x2E, 0x33,
/// 0x2E, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAB, 0x57] ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F4", subcode = "03")]
pub struct GameServerConnect {
  #[serde(with = "StringFixed::<typenum::U16>")]
  pub host: String,
  #[serde(with = "IntegerLE")]
  pub port: u16,
}

/// `C2:F4:06` — Represents a list of available Game Servers.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// count | `U16` | The number of
/// [GameServerListEntry](./struct.GameServerListEntry.html) in this response.
/// | LE entries | `GameServerListEntry[]` | An array of server entries. | -
///
/// ### Layout - entries
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// code | `U16` | The game server's code (segmented in 20 number intervals).
/// Maps to localized names on the client. If the 2 lowest bits are *both* set
/// or unset, marked as **Non-PvP**. | LE load | `U8` | The game server's load
/// balance, specified in ranges (**1-99:** load, **100-127:** full, **≥128:**
/// now preparing). | - unknown | `U8` | Unused by the game client. Fixed value
/// on retail (`0x66`). | -
///
/// ## Example
///
/// ```c
/// [0xC2, 0x00, 0x0B, 0xF4, 0x06, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00]
/// ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C2", code = "F4", subcode = "06")]
pub struct GameServerList(#[serde(with = "VectorLengthBE::<u16>")] pub Vec<GameServerListEntry>);

#[derive(Serialize, Debug)]
pub struct GameServerListEntry {
  pub code: GameServerCode,
  pub load: GameServerLoad,
  unknown: u8,
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

impl FromIterator<GameServerListEntry> for GameServerList {
  fn from_iter<I: IntoIterator<Item = GameServerListEntry>>(iter: I) -> Self {
    GameServerList(iter.into_iter().collect::<Vec<_>>())
  }
}
