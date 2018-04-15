use super::traits::{PacketSink, PacketStream};
use super::util;
use failure::Error;
use futures::prelude::*;
use murust_data_model::entities::Account;
use murust_service::ServiceManager;

mod lobby;
mod login;
mod world;

#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  manager: ServiceManager,
  stream: S,
) -> Result<S, Error> {
  // TODO: Log out account upon exit!
  let account_service = manager.account_service();
  let (account, stream) = await!(login::serve(account_service, stream))?;

  let account_service = manager.account_service();
  let session = serve_with_account(account.clone(), manager, stream);

  await!(session.then(move |future| {
    // Regardless of the session result, ensure the account is logged out
    match account_service.logout(&account) {
      Ok(_) => info!("Account '{}' logged out", &account.username),
      Err(error) => {
        error!("Failed to log out account '{}'", &account.username);
        debug!("{:#?}", error);
      },
    }
    future
  }))
}

#[async(boxed_send)]
pub fn serve_with_account<S: PacketStream + PacketSink + Send + 'static>(
  account: Account,
  manager: ServiceManager,
  stream: S,
) -> Result<S, Error> {
  let character_service = manager.character_service();
  let (character, stream) = await!(lobby::serve(account, character_service, stream))?;

  let stream = await!(world::serve(character, stream))?;
  Ok(stream)
}
