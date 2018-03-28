pub(crate) use self::join::JoinService;
pub(crate) use self::rpc::RpcService;
use std::net::SocketAddrV4;
use std::time::Duration;

mod join;
pub mod rpc;

/// An interface exposing the Join Service to the RPC service.
pub trait JoinServiceInterface: Sync + Send + 'static {
  /// Returns the Join Service's socket.
  fn socket(&self) -> SocketAddrV4;

  /// Returns the Join Service's client count.
  fn number_of_clients(&self) -> usize;

  /// Returns the Join Service's uptime.
  fn uptime(&self) -> Duration;
}
