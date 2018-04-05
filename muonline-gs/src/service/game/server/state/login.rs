use super::{PacketSink, PacketStream};
use controller::GameServerController;
use futures::prelude::*;
use mudb::AccountInterface;
use mupack::PacketDecodable;
use protocol;
use std::io;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  controller: GameServerController,
  stream: S,
) -> io::Result<S> {
  // TODO: Return InvalidData instead? Or chain the error
  // The next packet expected from the client is a login request
  let (packet, stream) = await!(stream.next_packet())?;
  let request = protocol::client::AccountLoginRequest::from_packet(&packet)?;

  let result = process_login(controller.clone(), &request);
  let stream = await!(stream.send_packet(&result))?;

  // Exit the login state if the client successfully authenticated
  if matches!(result, protocol::realm::AccountLoginResult::Success) {
    return Ok(stream);
  }

  // Recursively handle failed login requests
  await!(serve(controller, stream))
}

/// Returns a login result from a login request.
fn process_login(
  controller: GameServerController,
  request: &protocol::client::AccountLoginRequest,
) -> protocol::realm::AccountLoginResult {
  if !is_valid_client(request) {
    return protocol::realm::AccountLoginResult::InvalidGameVersion;
  }

  // TODO: Check if user is already connected
  // TODO: Inform the user if server is full from here?
  // TODO: Prevent login after too many failed attempts
  let auth = controller
    .database()
    .authenticate(&request.username, &request.password);

  match auth {
    Some(_account) => protocol::realm::AccountLoginResult::Success,
    None => protocol::realm::AccountLoginResult::InvalidAccount,
  }
}

fn is_valid_client(request: &protocol::client::AccountLoginRequest) -> bool {
  // TODO: Validate client version by using config
  request.version == protocol::shared::VERSION && request.serial == *b"ugkgameshield000"
}
