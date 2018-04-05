use self::traits::{PacketSink, SocketProvider};
use controller::GameServerController;
use futures::{Future, Stream, future::Either};
use std::io;
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use {mucodec, mupack, protocol, tokio};

mod login;
mod traits;

/// Starts the Game Server using the supplied controller.
pub fn serve(controller: GameServerController) -> io::Result<()> {
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
  let listener = TcpListener::bind(&controller.socket().into())?;

  // TODO: The public IP needs to be retrieved
  // Update the controller with the TCP port that's been bound
  controller.refresh_socket(listener.ipv4socket()?);

  let server = listener
    // Wait for incoming connections
    .incoming()
    // Process each new client connection
    .for_each(closet!([controller] move |stream| process_client(&controller, stream)))
    // Listen for any cancellation events from the controller
    .select(cancel);

  tokio::run(
    server
      .map(|(item, _)| item)
      .map_err(|(error, _)| error!("Game Service: {}", error)),
  );
  Ok(())
}

/// Setups and spawns a new task for a client.
fn process_client(controller: &GameServerController, stream: TcpStream) -> io::Result<()> {
  let controller = controller.clone();
  let socket = stream.ipv4socket()?;

  // Use a C3/C4 encrypted TCP codec
  let stream = stream.framed(codec());

  // TODO: Check if user is banned/server is preparing? Admin...
  let client = match controller.client_manager().add(socket) {
    // A slot has been allocated for the client
    Some(client_id) => {
      let future = stream
        // The client periodically sends time information
        .filter(client_packet_filter)
        // Inform the client of the success by providing its ID
        .send_packet(&protocol::realm::JoinResult::success(client_id as u16))
        // Hand over the control to the login module
        .and_then(closet!([controller] |stream| login::serve(controller, stream)))
        // Remove the client from the service state
        .then(move |future| {
          controller.client_manager().remove(client_id);
          future.map(|_| ())
        });
      Either::A(future)
    },
    // There are no free slots available for the client.
    None => {
      let future = stream
        .send_packet(&protocol::realm::JoinResult::Failure)
        .map(|_| ());
      Either::B(future)
    },
  };

  // Spawn each client on an executor
  tokio::spawn(client.map_err(|error| error!("Game Client: {}", error)));
  Ok(())
}

fn client_packet_filter(packet: &mupack::Packet) -> bool {
  match protocol::Client::from_packet(packet) {
    Ok(protocol::Client::ClientTime(_)) => false,
    _ => true,
  }
}

/// Returns the codec used for a Game Server.
fn codec() -> mucodec::PacketCodec {
  mucodec::PacketCodec::new(
    mucodec::State::new(None, Some(mupack::crypto::SERVER.clone())),
    mucodec::State::new(
      Some(&mupack::XOR_CIPHER),
      Some(mupack::crypto::CLIENT.clone()),
    ),
  )
}
