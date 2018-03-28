use super::context::JoinServiceContext;
use futures::future::Either;
use futures::sync::mpsc;
use futures::{Future, IntoFuture, Sink, Stream};
use mupack::{self, Packet, PacketEncodable};
use service::JoinServiceInterface;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use {mucodec, protocol, tokio};

#[derive(Clone)]
pub struct JoinServiceWorker {
  context: Arc<JoinServiceContext>,
  cancel_tx: mpsc::Sender<()>,
}

impl JoinServiceWorker {
  /// Constructs a new Join Service worker.
  pub fn new(socket: SocketAddrV4, cancel_tx: mpsc::Sender<()>) -> Self {
    let context = Arc::new(JoinServiceContext::new(socket));
    JoinServiceWorker { context, cancel_tx }
  }

  pub fn close(&self) -> io::Result<()> {
    self
      .cancel_tx
      .clone()
      .send(())
      .wait()
      .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "service already closed"))
      .map(|_| ())
  }

  /// Serves the Join Service worker.
  pub fn serve(self, cancel_rx: mpsc::Receiver<()>) -> io::Result<()> {
    let cancel = cancel_rx.into_future().map(|_| ()).map_err(|_| {
      io::Error::new(
        io::ErrorKind::BrokenPipe,
        "service close transmitter closed",
      )
    });

    let this = self.clone();
    let server = TcpListener::bind(&self.socket().into())?
      // Wait for incoming connections
      .incoming()
      // Process each new client connection
      .for_each(move |tcp| this.process_client(tcp))
      // Listen for cancellation event from the front-end
      .select(cancel)
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("<JoinServiceWorker> {:#?}", error));

    info!("Service running on port {}", self.socket().port());
    tokio::run(server);
    Ok(())
  }

  /// Processes a new client connection.
  fn process_client(&self, tcp: TcpStream) -> io::Result<()> {
    // Retrieve the address of the connected user
    let remote_addr = match tcp.peer_addr()? {
      SocketAddr::V4(socket) => socket,
      SocketAddr::V6(_) => unreachable!("invalid IP protocol"),
    };

    let client_id = self.add_client(remote_addr);
    info!("Client<{}> connected", remote_addr);

    // Use a non C3/C4 encrypted TCP codec
    let codec = mucodec::PacketCodec::new(
      mucodec::State::new(None, None),
      mucodec::State::new(Some(&mupack::XOR_CIPHER), None),
    );

    let (writer, reader) = tcp.framed(codec).split();
    let this = self.clone();

    // Provide a simple request-response service
    let connection = reader
      .fold(
        writer,
        clone_army!([this] move |writer, packet| {
        match this.process_packet(&packet) {
          Ok(packet) => Either::A(writer.send(packet)),
          Err(error) => Either::B(Err(error).into_future()),
        }
      }),
      )
      .map(|_| ())
      .or_else(move |error| {
        error!("Client<{}> error; {}", remote_addr, error);
        debug!("<serve> {:#?}", error);
        Ok(())
      })
      .map(move |_| {
        this.remove_client(client_id);
        info!("Client<{}> disconnected", remote_addr)
      });

    tokio::spawn(connection);
    Ok(())
  }

  /// Processes a new packet from the client.
  fn process_packet(&self, packet: &Packet) -> io::Result<Packet> {
    let client_packet = protocol::Client::from_packet(packet)?;
    debug!("<ProtoCore> {:#?}", client_packet);

    match client_packet {
      protocol::Client::JoinServerConnectRequest(version) => {
        // TODO: Do not hardcode the version
        if (version.major, version.minor, version.patch) == (0, 0, 1) {
          protocol::join::JoinServerConnectResult(true).to_packet()
        } else {
          Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "incorrect API version",
          ))
        }
      },
      // protocol::Client::GameServerConnectRequest(server) => {
      // },
      // protocol::Client::GameServerListRequest(_) => {
      //   use protocol::join::{GameServerList, GameServerListEntry};
      //   use protocol::{GameServerCode, GameServerLoad};
      //
      //   self
      //     .servers()
      //     .status()
      //     .collect()
      //     .map(|servers| servers
      //       .map(|server| GameServerListEntry::new(...))
      //       .collect::<GameServerList>()
      //       .to_packet()
      //     );
      // },
      _ => {
        let message = format!("unhandled packet {:x}", packet.code());
        Err(io::Error::new(io::ErrorKind::InvalidInput, message))
      },
    }
  }
}

impl Deref for JoinServiceWorker {
  type Target = JoinServiceContext;

  /// Returns the inner service context.
  fn deref(&self) -> &Self::Target { self.context.as_ref() }
}

impl JoinServiceInterface for JoinServiceWorker {
  /// Returns the socket used.
  fn socket(&self) -> SocketAddrV4 { self.context.socket() }

  /// Returns the number of clients.
  fn number_of_clients(&self) -> usize { self.context.number_of_clients() }

  /// Returns the current uptime.
  fn uptime(&self) -> Duration { self.context.uptime() }
}
