use super::{PacketSink, PacketStream};
use futures::prelude::*;
use muonline_packet::PacketDecodable;
use murust_data_model::entities::Account;
use murust_service::{AccountLoginError, AccountService};
use protocol::game::{client, server, VERSION};
use std::io;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  account_service: AccountService,
  stream: S,
) -> io::Result<(Account, S)> {
  // TODO: Return InvalidData instead? Or chain the error
  // The next packet expected from the client is a login request
  let (packet, stream) = await!(stream.next_packet())?;
  let request = client::AccountLoginRequest::from_packet(&packet)?;

  match process_login(&account_service, &request)? {
    Ok(account) => {
      // Inform of success and return the client's account
      let success = server::AccountLoginResult::Success;
      let stream = await!(stream.send_packet(&success))?;
      Ok((account, stream))
    },
    Err(result) => {
      // Recursively handle failed login requests
      let stream = await!(stream.send_packet(&result))?;
      await!(serve(account_service, stream))
    },
  }
}

/// Returns a login result from a login request.
fn process_login(
  account_service: &AccountService,
  request: &client::AccountLoginRequest,
) -> io::Result<Result<Account, server::AccountLoginResult>> {
  if !is_valid_client(request) {
    return Ok(Err(server::AccountLoginResult::InvalidGameVersion));
  }

  // TODO: Inform the user if server is full from here?
  let result = account_service
    .login(&request.username, &request.password)
    .map_err(|_| Into::<io::Error>::into(io::ErrorKind::Other))?
    .map_err(|error| match error {
      // TODO: It should not be revealed that the password was incorrect (only InvalidAccount)
      AccountLoginError::InvalidUsername => server::AccountLoginResult::InvalidAccount,
      AccountLoginError::InvalidPassword(_) => server::AccountLoginResult::IncorrectPassword,
      AccountLoginError::AlreadyConnected(_) => server::AccountLoginResult::AlreadyConnected,
      AccountLoginError::TooManyAttempts(_) => server::AccountLoginResult::TooManyAttempts,
    });
  Ok(result)
}

fn is_valid_client(request: &client::AccountLoginRequest) -> bool {
  // TODO: Validate client version by using config
  request.version == VERSION && request.serial == *b"ugkgameshield000"
}
