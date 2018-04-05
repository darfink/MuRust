use super::traits::{PacketSink, PacketStream};
use controller::GameServerController;
use futures::{Future, IntoFuture, future::Either};
use mudb::AccountInterface;
use mupack::PacketDecodable;
use protocol;
use std::io;

// TODO: Implement this in states using async + await
pub fn serve<S: PacketStream + PacketSink + Send + 'static>(
  controller: GameServerController,
  stream: S,
) -> Box<Future<Item = (), Error = io::Error> + Send> {
  let login = stream
    .next_packet()
    .and_then(closet!([controller] move |(packet, stream)| {
      // The next packet expected from the client is a login request
      match protocol::client::AccountLoginRequest::from_packet(&packet) {
        // If another packet is received it's not following protocol
        Err(_) => Either::A(Err(io::ErrorKind::InvalidData.into()).into_future()),
        Ok(request) => {
          // TODO: Validate client version
          let auth = controller.database().authenticate(&request.username, &request.password);
          let packet = match auth {
            Some(_account) => protocol::realm::AccountLoginResult::Success,
            None => protocol::realm::AccountLoginResult::InvalidAccount,
          };

          Either::B(stream.send_packet(&packet))
        },
      }
    }))
    .and_then(|stream| serve(controller, stream));
  Box::new(login)
}
