use super::{util, PacketSink, PacketStream};
use failure::Error;
use futures::prelude::*;
use murust_data_model::entities::Character;
use protocol::game::server;
use std::time::Duration;

#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  character: Character,
  mut stream: S,
) -> Result<S, Error> {
  stream = await!(stream.send_packet(&server::CharacterKillCount(character.player_kills as u8)))?;
  stream = await!(stream.send_packet(&server::CharacterInventory::new(&character)))?;

  await!(util::delay(Duration::from_secs(10)))?;
  Ok(stream)
}
