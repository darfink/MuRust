use context::GameServerContext;
use failure::{Error, Fail};
use futures::{prelude::*, sync::mpsc};
use listener::traits::{PacketSink, PacketStream};
use muonline_packet::Packet;
use murust_data_model::types::ObjectId;
use player::Player;
use std::io;
use views::PlayerView;

#[async(boxed_send)]
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  player_id: ObjectId,
  context: GameServerContext,
  stream: S,
) -> Result<(), Error> {
  let (server_sender, server_receiver) = mpsc::unbounded::<Packet>();
  let (client_writer, client_reader) = stream.split();

  // All packets to the client are sent via a channel
  let packets_to_client = server_receiver
    .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "sender channel closed"))
    .forward(client_writer)
    .map_err(|error| Error::from(error.context("Server transmission stream closed abrutply")))
    .map(|_| ());

  // Construct the player instance that will last throughout the session
  let mut player = Player::new(player_id, context, PlayerView::new(server_sender));

  // Process each incoming packet using the default client packet handler
  // TODO: Ugly clone for each incoming packet...
  let packets_to_server = client_reader
    .map_err(|error| Error::from(error.context("Server receiver stream closed abruptly")))
    .for_each(move |packet| {
      player
        .packet_handler
        .clone()
        .handle_packet(&mut player, packet)
    });

  let session = packets_to_client.select(packets_to_server);
  let session = session.map(|_| ()).map_err(|(error, _)| error);
  await!(session)
}
