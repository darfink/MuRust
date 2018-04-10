use GameServerBrowser;
use futures::{Future, IntoFuture, Stream, future::Either};
use muonline_packet::{Packet, PacketEncodable};
use protocol::connect::{self, server, Client};
use std::io;

pub fn proto_core(
  browser: &GameServerBrowser,
  packet: &Packet,
) -> impl Future<Item = Packet, Error = io::Error> + Send {
  let client_packet = match Client::from_packet(packet) {
    Err(error) => return Either::A(Err(error).into_future()),
    Ok(packet) => packet,
  };

  match client_packet {
    Client::ConnectServerRequest(request) => {
      let result = if request.version == connect::VERSION {
        server::ConnectServerResult::success().to_packet()
      } else {
        Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          "incorrect API version",
        ))
      };
      Either::A(result.into_future())
    },
    Client::GameServerConnectRequest(server) => {
      let result = browser.query(server.id).and_then(|status| {
        server::GameServerConnect::new(status.host.to_string(), status.port).to_packet()
      });
      Either::B(Box::new(result) as Box<Future<Item = Packet, Error = io::Error> + Send>)
    },
    Client::GameServerListRequest => {
      let result = browser
        .query_all()
        .map(|server| {
          (
            server.id,
            connect::models::ServerLoad::Load(server.load_factor()),
          )
        })
        .collect()
        .and_then(|servers| server::GameServerList::new(servers).to_packet());
      Either::B(Box::new(result) as Box<Future<Item = Packet, Error = io::Error> + Send>)
    },
    _ => {
      let message = format!("unhandled packet {:x}", packet.code());
      Either::A(Err(io::Error::new(io::ErrorKind::InvalidInput, message)).into_future())
    },
  }
}
