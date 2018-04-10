//! Connect Server Packets.
//!
//! ## Handshake
//!
//! The handshake is initialized by the client, and ensures that the client and
//! server uses a compatible protocol version.
//!
//! ## Example Session
//!
//! > Client → Server ([ConnectServerRequest](../client/struct.ConnectServerRequest.html))
//!
//! ```c
//! [0xC1, 0x06, 0xA9, 0x00, 0x00, 0x01]
//! ```
//!
//! > Server → Client
//! ([ConnectServerResult](./struct.ConnectServerResult.html))
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

use connect::models::ServerLoad;
use muonline_packet_serialize::{IntegerLE, StringFixed, VectorLengthBE};
use std::iter::IntoIterator;
use typenum;

/// `C1:00` — Describes the result of a [ConnectRequest](../client/struct.JoinServerConnectRequest.html).
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// result | `U8` | Integer representing the request result. | -
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0x00, 0x01]
/// ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "00")]
pub struct ConnectServerResult(bool);

impl ConnectServerResult {
  /// Returns a successful connect server result.
  pub fn success() -> Self { ConnectServerResult(true) }

  /// Returns an unsuccessful connect server result.
  pub fn failure() -> Self { ConnectServerResult(false) }
}

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
/// [0xC1, 0x16, 0xF4, 0x03, 0x38, 0x35, 0x2E, 0x32, 0x32, 0x36, 0x2E, 0x33, 0x2E, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAB, 0x57]
/// ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F4", subcode = "03")]
pub struct GameServerConnect {
  #[serde(with = "StringFixed::<typenum::U16>")]
  pub host: String,
  #[serde(with = "IntegerLE")]
  pub port: u16,
}

impl GameServerConnect {
  pub fn new<S: Into<String>>(host: S, port: u16) -> Self {
    GameServerConnect { host: host.into(), port }
  }
}

/// `C2:F4:06` — Represents a list of available Game Servers.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// count | `U16` | The number of [GameServerListEntry](./meta/struct.GameServerListEntry.html) in this response. | LE
/// entries | `GameServerListEntry[]` | An array of server entries. | -
///
/// ### Layout - entries
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// code | `U16` | The game server's code (segmented in 20 number intervals). Maps to localized names on the client. If the 2 lowest bits are *both* set or unset, marked as **Non-PvP**. | LE
/// load | `U8` | The game server's load balance, specified in ranges (**1-99:** load, **100-127:** full, **≥128:** now preparing). | -
/// unused | `U8` | Unused by the game client. Fixed value on retail (`0x77`). | -
///
/// ## Example
///
/// ```c
/// [0xC2, 0x00, 0x0B, 0xF4, 0x06, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00]
/// ```
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C2", code = "F4", subcode = "06")]
pub struct GameServerList(#[serde(with = "VectorLengthBE::<u16>")] Vec<GameServerListEntry>);

impl GameServerList {
  /// Constructs a new Game Server list from a tuple of server ID and load balance.
  pub fn new<I: IntoIterator<Item = (u16, ServerLoad)>>(servers: I) -> Self {
    let unused = 0x77;
    GameServerList(
      servers
        .into_iter()
        .map(|(id, load)| GameServerListEntry { id, load, unused })
        .collect::<Vec<_>>(),
    )
  }
}

/// A Game Server list entry.
#[derive(Serialize, Debug)]
struct GameServerListEntry {
  /// The Game Server's identifier.
  #[serde(with = "IntegerLE")]
  id: u16,
  /// The Game Server's load balance.
  load: ServerLoad,
  /// Unused field.
  unused: u8,
}
