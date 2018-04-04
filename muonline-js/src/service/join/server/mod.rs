use controller::JoinServerController;
use futures::{Future, Stream};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use {mucodec, mupack, tokio};

mod core;

/// Starts the Join Server using the supplied controller.
pub fn serve(controller: JoinServerController) -> io::Result<()> {
  let cancel = controller
    // The controller supplies the exit invoker
    .take_close_receiver()
    // A stream is not of any interest
    .into_future()
    // The close action can only be triggered once
    .map(|_| ())
    // An MPSC receiver cannot produce an error
    .map_err(|_| io::ErrorKind::Other.into());

  // Listen on the supplied TCP socket
  let server = TcpListener::bind(&controller.socket().into())?
    // Wait for incoming connections
    .incoming()
    // Process each new client connection
    .for_each(closet!([controller] move |stream| process_client(&controller, stream)))
    // Listen for any cancellation events from the controller
    .select(cancel);

  tokio::run(
    server
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("Join Service: {}", error)),
  );
  Ok(())
}

/// Setups and spawns a new task for a client.
fn process_client(controller: &JoinServerController, stream: TcpStream) -> io::Result<()> {
  // Add the client to the manager
  let id = controller.client_manager().add(ipv4socket(&stream)?);
  let controller = controller.clone();

  let (writer, reader) = stream
    // Use a non C3/C4 encrypted TCP codec
    .framed(codec())
    // Split the stream value into two separate handles
    .split();

  let client = reader
    // Each packet received maps to a response packet
    .and_then(closet!([controller] move |packet| core::proto_core(&controller, &packet)))
    // Return each response packet to the client
    .forward(writer)
    // Remove the client from the service state
    .then(move |future| {
      controller.client_manager().remove(id);
      future
    });

  // Spawn each client on an executor
  tokio::spawn(
    client
      .map(|_| ())
      .map_err(|error| error!("Join Client: {}", error)),
  );
  Ok(())
}

/// Returns the codec used for a Join Server.
fn codec() -> mucodec::PacketCodec {
  mucodec::PacketCodec::new(
    mucodec::State::new(None, None),
    mucodec::State::new(Some(&mupack::XOR_CIPHER), None),
  )
}

/// Returns the client's IPv4 socket if available.
fn ipv4socket(stream: &TcpStream) -> io::Result<SocketAddrV4> {
  match stream.peer_addr()? {
    SocketAddr::V4(socket) => Ok(socket),
    _ => Err(io::ErrorKind::InvalidInput.into()),
  }
}
