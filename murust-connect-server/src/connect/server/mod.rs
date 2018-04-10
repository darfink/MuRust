use futures::{Future, Stream, sync::mpsc};
use muonline_packet::XOR_CIPHER;
use muonline_packet_codec::{self, PacketCodec};
use std::{io, net::{Shutdown, SocketAddr, SocketAddrV4}};
use tokio::{self, io::AsyncRead, net::{TcpListener, TcpStream}};
use {ClientManager, GameServerBrowser};

mod core;

/// Starts serving the Connect Server
pub fn serve(
  socket: SocketAddrV4,
  server_browser: GameServerBrowser,
  client_manager: ClientManager,
  close_receiver: mpsc::Receiver<()>,
) -> io::Result<()> {
  // Augment the close receiver for our server future
  let cancel = close_receiver
    .into_future()
    .map(|_| ())
    .map_err(|_| io::ErrorKind::Other.into());

  // Listen on the supplied TCP socket
  let server = TcpListener::bind(&socket.into())?
    // Wait for incoming connections
    .incoming()
    // Process each new client connection
    .for_each(closet!([server_browser, client_manager] move |stream| {
      process_client(&server_browser, &client_manager, stream)
    }))
    // Listen for any cancellation events from the controller
    .select(cancel);

  tokio::run(
    server
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("Connect Service: {}", error)),
  );
  Ok(())
}

/// Setups and spawns a new task for a client.
fn process_client(
  browser: &GameServerBrowser,
  clients: &ClientManager,
  stream: TcpStream,
) -> io::Result<()> {
  // Try to add the client to the manager
  let id = match clients.add(ipv4socket(&stream)?) {
    Some(id) => id,
    None => {
      let _ = stream.shutdown(Shutdown::Both);
      return Ok(());
    },
  };

  let (writer, reader) = stream
    // Use a non C3/C4 encrypted TCP codec
    .framed(codec())
    // Split the stream value into two separate handles
    .split();

  let client = reader
    // Each packet received maps to a response packet
    .and_then(closet!([browser] move |packet| core::proto_core(&browser, &packet)))
    // Return each response packet to the client
    .forward(writer)
    // Remove the client from the service state
    .then(closet!([clients] move |future| {
      clients.remove(id);
      future
    }));

  // Spawn each client on an executor
  tokio::spawn(
    client
      .map(|_| ())
      .map_err(|error| error!("Connect Client: {}", error)),
  );
  Ok(())
}

/// Returns the codec used for a Connect Server.
fn codec() -> PacketCodec {
  PacketCodec::new(
    muonline_packet_codec::State::new(None, None),
    muonline_packet_codec::State::new(Some(&XOR_CIPHER), None),
  )
}

/// Returns the client's IPv4 socket if available.
fn ipv4socket(stream: &TcpStream) -> io::Result<SocketAddrV4> {
  match stream.peer_addr()? {
    SocketAddr::V4(socket) => Ok(socket),
    _ => Err(io::ErrorKind::InvalidInput.into()),
  }
}
