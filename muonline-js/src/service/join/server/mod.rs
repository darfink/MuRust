use super::JoinServiceControl;
use futures::{Future, IntoFuture, Stream};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use {mucodec, mupack, tokio};

mod core;

/// Starts the Join Server using the supplied state.
pub fn serve<S, C>(state: S, cancel: C) -> io::Result<()>
where
  S: JoinServiceControl,
  C: IntoFuture<Item = (), Error = io::Error>,
  C::Future: Send + 'static,
{
  // Listen on the supplied TCP socket
  let server = TcpListener::bind(&state.socket().into())?
    // Wait for incoming connections
    .incoming()
    // Process each new client connection
    .for_each(closet!([state] move |stream| process_client(&state, stream)))
    // Listen for any cancellation events from the front-end
    .select(cancel.into_future());

  tokio::run(
    server
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("Join Service: {}", error)),
  );
  Ok(())
}

/// Setups and spawns a new task for a client.
fn process_client<S: JoinServiceControl>(state: &S, stream: TcpStream) -> io::Result<()> {
  // Retrieve the client's address and store it
  let id = state.add_client(ipv4socket(&stream)?);
  let state = state.clone();

  let (writer, reader) = stream
    // Use a non C3/C4 encrypted TCP codec
    .framed(codec())
    // Split the stream value into two separate handles
    .split();

  let client = reader
    // Each packet received maps to a response packet
    .and_then(closet!([state] move |packet| core::proto_core(&state, &packet)))
    // Return each response packet to the client
    .forward(writer)
    // Remove the client from the service state
    .then(move |future| {
      state.remove_client(id);
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
