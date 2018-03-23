//! Game Client

use std::io;
use std::ops::Deref;
use muonline_packet::{Packet, PacketType, PacketDecodable};
use GameServerCode;

/// An aggregation of all possible client packets.
#[derive(Debug)]
pub enum Client {
  JoinServerConnectRequest(JoinServerConnectRequest),
  GameServerConnectRequest(GameServerConnectRequest),
  GameServerListRequest(GameServerListRequest),
  None,
}

impl Client {
  /// Constructs a client packet from an unidentified one.
  pub fn from_packet(packet: &Packet) -> io::Result<Self> {
    // TODO: Handle this boilerplate
    match (packet.code(), packet.data()) {
      // TODO: Subcodes should be automatic
      (JoinServerConnectRequest::CODE, _) => JoinServerConnectRequest::from_packet(packet).map(Client::JoinServerConnectRequest),
      (GameServerConnectRequest::CODE, &[0x03, ..]) => GameServerConnectRequest::from_packet(packet).map(Client::GameServerConnectRequest),
      (GameServerListRequest::CODE, &[0x06, ..]) => GameServerListRequest::from_packet(packet).map(Client::GameServerListRequest),
      _ => Ok(Client::None),
    }
  }
}

/// `C1:A9` - Connect request to a Join Server.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// major | `U8` | Integer representing the type of message. | -
/// minor | `U8` | Integer representing the type of message. | -
/// patch | `U8` | Integer representing the type of message. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "A9")]
pub struct JoinServerConnectRequest {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

/// `C1:F4:03` - Request for a Game Server's connection information.
///
/// This is sent to the Join Server, not the Game Server.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// code | `U16` | The selected server's code. | LE
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F4", subcode = "03")]
pub struct GameServerConnectRequest {
  pub code: GameServerCode,
}

impl Deref for GameServerConnectRequest {
  type Target = GameServerCode;

  fn deref(&self) -> &Self::Target {
    &self.code
  }
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
