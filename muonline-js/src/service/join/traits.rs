use futures::Stream;
use mugs;
use service::JoinServiceListen;
use std::io;
use std::net::SocketAddrV4;

pub trait JoinServiceControl: JoinServiceListen + Clone {
  /// Adds a new client to the state.
  fn add_client(&self, socket: SocketAddrV4) -> usize;

  /// Removes a client from the state.
  fn remove_client(&self, id: usize);

  // TODO: This should not be boxed (i.e don't use a trait?)
  /// Queries all available game servers.
  fn query_game_servers(
    &self,
  ) -> Box<Stream<Item = mugs::rpc::GameServerStatus, Error = io::Error> + Send>;
}
