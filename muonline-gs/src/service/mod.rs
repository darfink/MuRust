pub use self::game::GameService;
pub use self::rpc::RpcService;
use std::net::SocketAddrV4;
use std::time::Duration;

mod game;
pub mod rpc;

pub trait GameServiceListen: Sync + Send + 'static {
  /// Returns the service's socket.
  fn socket(&self) -> SocketAddrV4;
}

/// An interface exposing the Game Service to the RPC service.
pub trait GameServiceInterface: GameServiceListen {
  /// Returns the service's id.
  fn id(&self) -> u16;

  /// Returns the service's client capacity.
  fn capacity(&self) -> usize;

  /// Returns the service's client count.
  fn number_of_clients(&self) -> usize;

  /// Returns the service's uptime.
  fn uptime(&self) -> Duration;
}
