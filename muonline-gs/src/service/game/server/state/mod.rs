use super::traits::{PacketSink, PacketStream};
use super::util;
use controller::GameServerController;
use futures::prelude::*;
use std::io;

pub mod lobby;
pub mod login;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  controller: GameServerController,
  stream: S,
) -> io::Result<S> {
  let stream = await!(login::serve(controller.clone(), stream))?;
  let stream = await!(lobby::serve(controller.clone(), stream))?;
  Ok(stream)
}
