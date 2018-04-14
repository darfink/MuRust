use self::traits::{PacketSink, SocketProvider};
use futures::{Future, Stream, future::Either, sync::mpsc};
use muonline_packet::{crypto, Packet, XOR_CIPHER};
use muonline_packet_codec::{self, PacketCodec};
use murust_service::ServiceManager;
use protocol::game::{server, Client};
use std::io;
use tokio::{self, io::AsyncRead, net::{TcpListener, TcpStream}};
use {ClientManager, ServerInfo};

mod state;
mod traits;
mod util;

/// Starts serving the Game Server
pub fn serve(
  server_info: ServerInfo,
  service_manager: ServiceManager,
  client_manager: ClientManager,
  close_receiver: mpsc::Receiver<()>,
) -> io::Result<()> {
  // Augment the close receiver for our server future
  let cancel = close_receiver
    .into_future()
    .map(|_| ())
    .map_err(|_| io::ErrorKind::Other.into());

  // Listen on the supplied TCP socket
  let listener = TcpListener::bind(&server_info.socket().into())?;

  // Update the server control with the TCP port that's been bound
  server_info.refresh_socket(listener.ipv4socket()?);

  let server = listener
    // Wait for incoming connections
    .incoming()
    // Process each new client connection
    .for_each(closet!([service_manager, client_manager] move |stream| {
      process_client(&service_manager, &client_manager, stream)
    }))
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
fn process_client(
  service_manager: &ServiceManager,
  clients: &ClientManager,
  stream: TcpStream,
) -> io::Result<()> {
  // Retrieve the client's socket address
  let socket = stream.ipv4socket()?;

  // Use a C3/C4 encrypted TCP codec
  let stream = stream.framed(codec());

  // TODO: Check if user is banned/server is preparing? Admin...
  let client = match clients.add(socket) {
    // A slot has been allocated for the client
    Some(client_id) => {
      let future = stream
        // The client periodically sends time information
        .filter(client_packet_filter)
        // Inform the client of the success by providing its ID
        .send_packet(&server::JoinResult::success(client_id as u16))
        // Let the state manager handle the life cycle of the session
        .and_then(closet!([service_manager] |stream| state::serve(service_manager, stream)))
        // Remove the client from the service state
        .then(closet!([clients] move |future| {
          clients.remove(client_id);
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
  tokio::spawn(client.map_err(|error| error!("Game Client: {}", error)));
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

fn client_packet_filter(packet: &Packet) -> bool {
  match Client::from_packet(packet) {
    Ok(Client::ClientTime(_)) => false,
    _ => true,
  }
}
