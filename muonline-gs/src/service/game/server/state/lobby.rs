use super::{util, PacketSink, PacketStream};
use controller::GameServerController;
use futures::prelude::*;
use protocol;
use std::io;
use std::time::Duration;

#[allow(unused_unsafe)]
#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  _controller: GameServerController,
  stream: S,
) -> io::Result<S> {
  // The lobby processes one packet at a time.
  let (packet, stream) = await!(stream.next_packet())?;

  match protocol::Client::from_packet(&packet)? {
    protocol::Client::CharacterListRequest => {
      let characters = protocol::realm::CharacterList::default();
      let stream = await!(stream.send_packet(&characters))?;

      // The client might crash unless there's a delay between these packets
      await!(util::delay(Duration::from_millis(250)))?;

      let motd = "Douglas you are handsome :D";
      let motd = protocol::realm::Message::Notice(motd.into());
      let stream = await!(stream.send_packet(&motd))?;
      await!(util::delay(Duration::from_secs(100)))?;
      Ok(stream)
    },
    _ => Err(io::ErrorKind::InvalidData.into()),
  }
}
