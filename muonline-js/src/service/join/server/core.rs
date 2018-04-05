use controller::JoinServerController;
use futures::{Future, IntoFuture, Stream, future::Either};
use mupack::{self, PacketEncodable};
use protocol;
use std::io;

// TODO: Removing boxing in this function
pub fn proto_core(
  controller: &JoinServerController,
  packet: &mupack::Packet,
) -> impl Future<Item = mupack::Packet, Error = io::Error> + Send {
  let client_packet = match protocol::Client::from_packet(packet) {
    Err(error) => return Either::A(Err(error).into_future()),
    Ok(packet) => packet,
  };

  match client_packet {
    protocol::Client::JoinServerConnectRequest(version) => {
      let result = if (version.major, version.minor, version.patch) == protocol::join::VERSION {
        protocol::join::JoinServerConnectResult(true).to_packet()
      } else {
        Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          "incorrect API version",
        ))
      };
      Either::A(result.into_future())
    },
    protocol::Client::GameServerConnectRequest(server) => {
      let result = controller
        .server_browser()
        .query(server.id)
        .and_then(|status| {
          protocol::join::GameServerConnect {
            host: status.host.to_string(),
            port: status.port,
          }.to_packet()
        });
      Either::B(Box::new(result) as Box<Future<Item = mupack::Packet, Error = io::Error> + Send>)
    },
    protocol::Client::GameServerListRequest => {
      let result = controller
        .server_browser()
        .query_all()
        .map(|game_server| {
          protocol::join::meta::GameServerListEntry::new(game_server.id, game_server.load_factor())
        })
        .collect()
        .and_then(|entries| protocol::join::GameServerList(entries).to_packet());
      Either::B(Box::new(result) as Box<Future<Item = mupack::Packet, Error = io::Error> + Send>)
    },
    _ => {
      let message = format!("unhandled packet {:x}", packet.code());
      Either::A(Err(io::Error::new(io::ErrorKind::InvalidInput, message)).into_future())
    },
  }
}
