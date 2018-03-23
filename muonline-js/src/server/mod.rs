use std::sync::Arc;
use std::{io, thread, thread::JoinHandle};
use std::net::{SocketAddr, SocketAddrV4};
use {log, mucodec, mupack};
use futures::future::Either;
use futures::sync::oneshot;
use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;
use ipc::Ipc;
pub use self::builder::Builder;

mod builder;
mod service;

struct GameServer {
  code: u16,
}

pub struct JoinServer {
  socket: SocketAddrV4,
  servers: Vec<GameServer>,
  cancel: oneshot::Receiver<()>,
  ipc: Arc<Ipc>,
}

impl JoinServer {
  /// Returns the Join Server builder.
  pub fn builder(socket: SocketAddrV4) -> Builder {
    Builder::new(socket)
  }

  /// Starts the Join Server using the configured settings.
  pub fn serve(self) -> JoinHandle<io::Result<()>> {
    thread::spawn(move || {
      let ipc = self.ipc.clone();
      self.serve_impl()?;
      ipc.on_exit();
      Ok(())
    })
  }

  fn serve_impl(self) -> io::Result<()> {
    let socket = self.socket;
    let ipc = self.ipc.clone();

    info!("Binding server to {}...", socket);
    log::logger().flush();

    let listener = TcpListener::bind(&SocketAddr::V4(socket))?;

    let server = listener.incoming()
    .for_each(move |tcp| {
      // Retrieve the address of the connected user
      let remote_addr = tcp.peer_addr()?;
      ipc.on_connect(remote_addr);
      info!("Client<{}> connect", remote_addr);

      // Use a non C3/C4 encrypted TCP codec
      let codec = mucodec::PacketCodec::new(
        mucodec::State::new(None, None),
        mucodec::State::new(Some(&mupack::XOR_CIPHER), None));

      let (writer, reader) = tcp.framed(codec).split();

      // Provide a simple request-response service
      let connection = reader
        .fold(writer, |writer, packet| {
          match service::protocol_core(packet) {
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
        .map(clone_army!([ipc] move |_| {
          info!("Client<{}> disconnect", remote_addr);
          ipc.on_disconnect(remote_addr);
        }));

      tokio::spawn(connection);
      Ok(())
    })
    // Listen for a potential cancellation event from the front-end
    .select(self.cancel.map_err(|error| io::Error::new(io::ErrorKind::BrokenPipe, error)))
    .map(|(item, _)| item)
    .map_err(|(error, _)| {
      error!("Server error; {}", error);
      debug!("<serve> {:#?}", error);
    });

    info!("Server running on port {}", socket.port());
    Ok(tokio::run(server))
  }
}