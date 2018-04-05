//! Game Client Packets

use StringFixedCredentials;
use muonline_packet::{Packet, PacketDecodable, PacketType};
use muserialize::IntegerLE;
use shared::{Serial, Version};
use std::io;
use typenum;

// TODO: Box the largest packets to decrease total size?
/// An aggregation of all possible client packets.
#[derive(Debug)]
pub enum Client {
  ClientTime(ClientTime),
  JoinServerConnectRequest(JoinServerConnectRequest),
  AccountLoginRequest(AccountLoginRequest),
  CharacterListRequest,
  GameServerConnectRequest(GameServerConnectRequest),
  GameServerListRequest,
  None,
}

impl Client {
  /// Constructs a client packet from an unidentified one.
  pub fn from_packet(packet: &Packet) -> io::Result<Self> {
    // TODO: Handle this boilerplate, subcodes should also be automatic
    match (packet.code(), packet.data()) {
      (ClientTime::CODE, &[0x00, _..]) => ClientTime::from_packet(packet).map(Client::ClientTime),
      (JoinServerConnectRequest::CODE, _) => {
        JoinServerConnectRequest::from_packet(packet).map(Client::JoinServerConnectRequest)
      },
      (AccountLoginRequest::CODE, &[0x01, _..]) => {
        AccountLoginRequest::from_packet(packet).map(Client::AccountLoginRequest)
      },
      (CharacterListRequest::CODE, &[0x00, _..]) => {
        CharacterListRequest::from_packet(packet).map(|_| Client::CharacterListRequest)
      },
      (GameServerConnectRequest::CODE, &[0x03, _..]) => {
        GameServerConnectRequest::from_packet(packet).map(Client::GameServerConnectRequest)
      },
      (GameServerListRequest::CODE, &[0x06, _..]) => {
        GameServerListRequest::from_packet(packet).map(|_| Client::GameServerListRequest)
      },
      _ => Ok(Client::None),
    }
  }
}

/// `C1:0E:00` - Local client timing values.
///
/// This is sent by default every 20th second.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// time | `U32` | The client's time instant in milliseconds. | LE
/// speed (attack) | `U16` | The client's current attack speed. | LE
/// speed (magic) | `U16` | The client's current magic speed. | LE
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "0E", subcode = "00")]
pub struct ClientTime {
  #[serde(with = "IntegerLE")]
  pub time: u32,
  #[serde(with = "IntegerLE")]
  pub attack_speed: u16,
  #[serde(with = "IntegerLE")]
  pub magic_speed: u16,
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

/// `C1:F1:01` - Authentication request sent upon client login.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// username | `CHAR(10)` | The specified username. | -
/// password | `CHAR(10)` | The specified password. | -
/// time | `U32` | The client's time instant in milliseconds. | LE
/// version | `U8(5)` | The client's protocol version. | -
/// serial | `CHAR(16)` | The client's serial version. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F1", subcode = "01")]
pub struct AccountLoginRequest {
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub username: String,
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub password: String,
  #[serde(with = "IntegerLE")]
  pub time: u32,
  pub version: Version,
  pub serial: Serial,
}

/// `C1:F3:00` - Request for an account's characters.
///
/// This is sent from the client as soon as it has successfully logged in with an
/// account.
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0xF3, 0x00]
/// ```
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "00")]
pub struct CharacterListRequest;

/// `C1:F4:03` - Request for a Game Server's connection information.
///
/// This is sent to the Join Server, not the Game Server.
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
