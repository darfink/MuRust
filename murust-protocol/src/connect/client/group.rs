use super::*;
use muonline_packet::{Packet, PacketDecodable, PacketType};
use std::io;

/// An aggregation of all possible client packets.
#[derive(Debug)]
pub enum Client {
  ConnectServerRequest(ConnectServerRequest),
  GameServerConnectRequest(GameServerConnectRequest),
  GameServerListRequest,
  None,
}

impl Client {
  /// Constructs a client packet from an unidentified one.
  pub fn from_packet(packet: &Packet) -> io::Result<Self> {
    // TODO: Handle this boilerplate, subcodes should also be automatic
    match (packet.code(), packet.data()) {
      (ConnectServerRequest::CODE, _) => {
        ConnectServerRequest::from_packet(packet).map(Client::ConnectServerRequest)
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
