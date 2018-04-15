use super::traits::{PacketSink, PacketStream};
use super::util;
use failure::Error;
use futures::prelude::*;
use murust_service::ServiceManager;

pub mod lobby;
pub mod login;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  service_manager: ServiceManager,
  stream: S,
) -> Result<S, Error> {
  // TODO: Log out account upon exit!
  let (account, stream) = await!(login::serve(service_manager.account_service(), stream))?;
  let (_character, stream) = await!(lobby::serve(
    account,
    service_manager.character_service(),
    stream,
  ))?;
  Ok(stream)
}
