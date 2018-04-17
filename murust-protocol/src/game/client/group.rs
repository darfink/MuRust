use super::*;
use muonline_packet::{Packet, PacketDecodable, PacketType};
use std::io;

/// An aggregation of all possible client packets.
#[derive(Debug)]
pub enum Client {
  ClientTime(ClientTime),
  CharacterAction(CharacterAction),
  // ItemMove(ItemMove),
  CharacterMove(CharacterMove),
  AccountLoginRequest(AccountLoginRequest),
  CharacterListRequest,
  CharacterCreate(CharacterCreate),
  CharacterDelete(CharacterDelete),
  CharacterSelect(CharacterSelect),
  Unknown,
}

impl Client {
  /// Constructs a client packet from an unidentified one.
  pub fn from_packet(packet: &Packet) -> io::Result<Self> {
    // TODO: Box the largest packets to decrease total size?
    // TODO: Handle this boilerplate, subcodes should also be automatic
    match (packet.code(), packet.data()) {
      (ClientTime::CODE, &[0x00, _..]) => ClientTime::from_packet(packet).map(Client::ClientTime),
      (CharacterAction::CODE, _) => {
        CharacterAction::from_packet(packet).map(Client::CharacterAction)
      },
      //(ItemMove::CODE, _) => ItemMove::from_packet(packet).map(Client::ItemMove),
      (CharacterMove::CODE, _) => CharacterMove::from_packet(packet).map(Client::CharacterMove),
      (AccountLoginRequest::CODE, &[0x01, _..]) => {
        AccountLoginRequest::from_packet(packet).map(Client::AccountLoginRequest)
      },
      (CharacterListRequest::CODE, &[0x00, _..]) => {
        CharacterListRequest::from_packet(packet).map(|_| Client::CharacterListRequest)
      },
      (CharacterCreate::CODE, &[0x01, _..]) => {
        CharacterCreate::from_packet(packet).map(Client::CharacterCreate)
      },
      (CharacterDelete::CODE, &[0x02, _..]) => {
        CharacterDelete::from_packet(packet).map(Client::CharacterDelete)
      },
      (CharacterSelect::CODE, &[0x03, _..]) => {
        CharacterSelect::from_packet(packet).map(Client::CharacterSelect)
      },
      _ => Ok(Client::Unknown),
    }
  }

  /// Returns whether this is an unknown packet or not.
  pub fn is_unknown(&self) -> bool {
    match *self {
      Client::Unknown => true,
      _ => false,
    }
  }
}
