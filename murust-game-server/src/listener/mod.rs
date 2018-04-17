use context::GameServerContext;
use failure::{Context, Error, Fail, ResultExt};
use futures::{Future, Stream, future::Either, sync::mpsc};
use listener::traits::{PacketSink, SocketProvider};
use muonline_packet::{crypto, XOR_CIPHER};
use muonline_packet_codec::{self, PacketCodec};
use protocol::game::server;
use tokio::{self, io::AsyncRead, net::{TcpListener, TcpStream}};

mod client;
mod traits;

/// Starts serving the Game Server
pub fn listen(context: GameServerContext, close_receiver: mpsc::Receiver<()>) -> Result<(), Error> {
  // Augment the close receiver for our server future
  let cancel = close_receiver
    .into_future()
    .map(|_| ())
    .map_err(|_| Context::new("Server exit receiver endpoint closed abruptly").into());

  // Listen on the supplied TCP socket
  let listener =
    TcpListener::bind(&context.socket().into()).context("Failed to bind server socket")?;

  // Update the server control with the TCP port that's been bound
  context.refresh_socket(listener.ipv4socket()?);

  let server = listener
    .incoming()
    .map_err(|error| error.context("Failed to listen for incoming connections").into())
    // Process each incoming connection as a new client
    .for_each(closet!([context] move |stream| process_client(&context, stream)))
    // Listen for any cancellation events from the controller
    .select(cancel);

  tokio::run(
    server
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("Game Listener: {}", error)),
  );
  Ok(())
}

/// Setups and spawns a new task for a client.
fn process_client(context: &GameServerContext, stream: TcpStream) -> Result<(), Error> {
  // Retrieve the client's socket address
  let socket = stream.ipv4socket()?;
  let stream = stream
    // Use a C3/C4 encrypted TCP codec
    .framed(codec())
    // Contextualize any errors produced
    .map_err(|error| error.context("Client stream failed").into());

  let client = match context.add_client(socket) {
    // A slot has been allocated for the client
    Some(client_id) => {
      let future = stream
        // TODO: Protocol details should not be expsoed here?
        // Inform the client of the success by providing its ID
        .send_packet(&server::JoinResult::success(client_id))
        // Let the state manager handle the life cycle of the session
        .and_then(closet!([context] move |stream| client::serve(client_id, context, stream)))
        // Remove the client from the server state
        .then(closet!([context] move |future| {
          context.remove_client(client_id);
          future.map(|_| ())
        }));
      Either::A(future)
    },
    // There are no free slots available for the client.
    None => {
      let future = stream.send_packet(&server::JoinResult::Failure).map(|_| ());
      Either::B(future)
    },
  };

  // Spawn each client on an executor
  tokio::spawn(client.map_err(|error| {
    error!("<Client> {}", error);
    debug!("{:#?}", error)
  }));
  Ok(())
}

/// Returns the codec used for a Game Server.
fn codec() -> PacketCodec {
  // TODO: Load the crypto files dynamically
  PacketCodec::new(
    muonline_packet_codec::State::new(None, Some(crypto::SERVER.clone())),
    muonline_packet_codec::State::new(Some(&XOR_CIPHER), Some(crypto::CLIENT.clone())),
  )
}
