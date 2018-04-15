use super::traits::{PacketSink, PacketStream};
use super::util;
use failure::Error;
use futures::prelude::*;
use murust_service::ServiceManager;

mod lobby;
mod login;
mod world;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  service_manager: ServiceManager,
  stream: S,
) -> Result<S, Error> {
  // TODO: Log out account upon exit!
  let (account, stream) = await!(login::serve(service_manager.account_service(), stream))?;

  let character_service = service_manager.character_service();
  let (character, stream) = await!(lobby::serve(account, character_service, stream))?;

  let stream = await!(world::serve(character, stream))?;
  Ok(stream)
}
