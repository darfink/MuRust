use service::GameServiceListen;
use std::net::SocketAddrV4;

pub trait GameServiceControl: GameServiceListen + Clone {
  /// Adds a new client to the state.
  fn add_client(&self, socket: SocketAddrV4) -> usize;

  /// Removes a client from the state.
  fn remove_client(&self, id: usize);
}
