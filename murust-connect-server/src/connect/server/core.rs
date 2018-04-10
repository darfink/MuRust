use GameServerBrowser;
use futures::Stream;
use futures::prelude::*;
use muonline_packet::{Packet, PacketEncodable};
use protocol::connect::{self, server, Client};
use std::io;

#[async(boxed_send)]
pub fn proto_core(browser: GameServerBrowser, packet: Packet) -> io::Result<Packet> {
  match Client::from_packet(&packet)? {
    Client::ConnectServerRequest(request) => {
      if request.version == connect::VERSION {
        server::ConnectServerResult::success().to_packet()
      } else {
        let message = "incorrect API version";
        Err(io::Error::new(io::ErrorKind::InvalidInput, message))
      }
    },
    Client::GameServerConnectRequest(server) => {
      let status = await!(browser.query(server.id))?;
      server::GameServerConnect::new(status.host.to_string(), status.port).to_packet()
    },
    Client::GameServerListRequest => await!(browser.query_all().collect())?
      .iter()
      .map(|server| (server.id, server.load_factor().into()))
      .collect::<server::GameServerList>()
      .to_packet(),
    _ => {
      let message = format!("unhandled packet {:x}", packet.code());
      Err(io::Error::new(io::ErrorKind::InvalidInput, message))
    },
  }
}
