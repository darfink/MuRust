use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::Either;
use futures::sync::oneshot;

use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

use mupack::{self, Packet, PacketEncodable};
use {mucodec, log};

use service::JoinServiceContext;

pub fn serve(
    context: Arc<JoinServiceContext>,
    cancel: oneshot::Receiver<()>) -> io::Result<()> {
  let socket = context.socket();

  info!("Binding service to {}...", socket);
  log::logger().flush();

  let listener = TcpListener::bind(&SocketAddr::V4(socket))?;
  let server = listener.incoming()
  .for_each(move |tcp| {
    // Retrieve the address of the connected user
    let remote_addr = tcp.peer_addr()?;
    let client_id = context.add_client(remote_addr);
    info!("Client<{}> connect", remote_addr);

    // Use a non C3/C4 encrypted TCP codec
    let codec = mucodec::PacketCodec::new(
      mucodec::State::new(None, None),
      mucodec::State::new(Some(&mupack::XOR_CIPHER), None));

    let (writer, reader) = tcp.framed(codec).split();

    // Provide a simple request-response service
    let connection = reader
      .fold(writer, |writer, packet| {
        match protocol_core(packet) {
          Ok(packet) => Either::A(writer.send(packet)),
          Err(error) => Either::B(Err(error).into_future()),
        }
      })
      .map(|_| ())
      .or_else(move |error| {
        error!("Client<{}> error; {}", remote_addr, error);
        debug!("<serve> {:#?}", error);
        Ok(())
      })
      .map(clone_army!([context] move |_| {
        context.remove_client(client_id);
        info!("Client<{}> disconnect", remote_addr)
      }));

    tokio::spawn(connection);
    Ok(())
  })
  // Listen for a potential cancellation event from the front-end
  .select(cancel.map_err(|error| io::Error::new(io::ErrorKind::BrokenPipe, error)))
  .map(|(item, _)| item)
  .map_err(|(error, _)| {
    error!("Server error; {}", error);
    debug!("<serve> {:#?}", error);
  });

  info!("Service running on port {}", socket.port());
  Ok(tokio::run(server))
}

fn protocol_core(packet: Packet) -> io::Result<Packet> {
  use protocol;

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
