use super::{util, PacketSink, PacketStream};
use futures::prelude::*;
use murust_data_model::entities::{Account, Character};
use murust_data_model::types::Class;
use murust_service::CharacterService;
use protocol::game::{server, Client};
use std::io;
use std::time::Duration;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  (account, character_service): (Account, CharacterService),
  stream: S,
) -> io::Result<(Character, S)> {
  let characters = character_service
    .find_by_account_id(account.id)
    .map_err(|_| Into::<io::Error>::into(io::ErrorKind::Other))?;

  // Process one incoming packet at a time.
  let (packet, stream) = await!(stream.next_packet())?;

  match Client::from_packet(&packet)? {
    Client::CharacterListRequest => {
      let list = server::CharacterList::new(Class::FairyElf, &characters);
      let stream = await!(stream.send_packet(&list))?;

      // The client might crash unless there's a delay between these packets
      await!(util::delay(Duration::from_millis(250)))?;

      let motd = "Welcome to Mu Online in Rust!";
      let motd = server::Message::Notice(motd.into());
      let stream = await!(stream.send_packet(&motd))?;
      await!(util::delay(Duration::from_secs(100)))?;
      Ok((characters.into_iter().next().unwrap(), stream))
    },
    _ => Err(io::ErrorKind::InvalidData.into()),
  }
}
