use controller::JoinServerController;
use futures::{Future, IntoFuture, Stream, future::Either};
use mupack::{self, PacketEncodable};
use protocol;
use std::io;

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
    // protocol::Client::GameServerConnectRequest(server) => {
    // },
    protocol::Client::GameServerListRequest => {
      let result = controller
        .browser()
        .query_all()
        .map(|status| {
          protocol::join::meta::GameServerListEntry::new(
            status.id.into(),
            protocol::model::GameServerLoad::Load(0.5),
          )
        })
        .collect()
        .and_then(|entries| protocol::join::GameServerList(entries).to_packet());
      Either::B(result)
    },
    _ => {
      let message = format!("unhandled packet {:x}", packet.code());
      Either::A(Err(io::Error::new(io::ErrorKind::InvalidInput, message)).into_future())
    },
  }
}
