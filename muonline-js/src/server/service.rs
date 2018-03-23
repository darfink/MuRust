use std::io;
use mupack::{Packet, PacketEncodable};
use protocol;

pub fn protocol_core(packet: Packet) -> io::Result<Packet> {
  let client_packet = protocol::Client::from_packet(&packet)?;
  debug!("<ProtoCore> {:#?}", client_packet);

  match client_packet {
    protocol::Client::JoinServerConnectRequest(version) => {
      // TODO: Do not hardcode the version
      if (version.major, version.minor, version.patch) == (0, 0, 1) {
        protocol::join::JoinServerConnectResult(true).to_packet()
      } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "incorrect API version"))
      }
    },
    //protocol::Client::GameServerConnectRequest(server) => {
    //},
    protocol::Client::GameServerListRequest(_) => {
      use protocol::{GameServerCode, GameServerLoad};
      use protocol::join::{GameServerList, GameServerListEntry};

      (0..1)
        .map(|_| GameServerListEntry::new(GameServerCode::new(1, 1), GameServerLoad::Load(0.5)))
        .collect::<GameServerList>()
        .to_packet()
    },
    _ => {
      let message = format!("invalid packet {:x}", packet.code());
      Err(io::Error::new(io::ErrorKind::InvalidInput, message))
    },
  }
}