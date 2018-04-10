//! Client Packets to the Connect Server.

pub use self::group::Client;
use super::Version;
use muonline_packet_serialize::IntegerLE;

mod group;

/// `C1:A9` - Syncrhonize request to a Connect Server.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// major | `U8` | The major connect protocol version. | -
/// minor | `U8` | The minor connect protocol version. | -
/// patch | `U8` | The patch connect protocol version. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "A9")]
pub struct ConnectServerRequest {
  pub version: Version,
}

/// `C1:F4:03` - Request for a Game Server's connection information.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// id | `U16` | The selected server's id. | LE
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F4", subcode = "03")]
pub struct GameServerConnectRequest {
  #[serde(with = "IntegerLE")]
  pub id: u16,
}

/// `C1:F4:06` - Request for the Game Server list.
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0xF4, 0x06]
/// ```
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F4", subcode = "06")]
pub struct GameServerListRequest;
