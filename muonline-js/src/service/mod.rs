pub use self::join::JoinService;
pub use self::rpc::RpcService;
use mugs;
use std::net::SocketAddrV4;
use std::time::Duration;

mod join;
pub mod rpc;

pub trait JoinServiceListen: Sync + Send + 'static {
  /// Returns the service's socket.
  fn socket(&self) -> SocketAddrV4;
}

/// An interface exposing the Join Service to the RPC service.
pub trait JoinServiceInterface: JoinServiceListen {
  /// Returns the service's client count.
  fn number_of_clients(&self) -> usize;

  /// Returns the service's uptime.
  fn uptime(&self) -> Duration;
}

/// An interface for queryable servers.
pub trait QueryableGameServer {
  /// Returns the URI of the Game Server.
  fn uri(&self) -> &str;
}

/// An implementation for remote servers.
impl QueryableGameServer for String {
  fn uri(&self) -> &str { self.as_ref() }
}

/// An implementation for local servers.
impl QueryableGameServer for mugs::GameServer {
  fn uri(&self) -> &str { self.uri() }
}
