use ClientManager;
use jsonrpc_core::Error;
use rpc::api::{ConnectServerApi, ConnectServerStatus};
use std::{net::SocketAddrV4, time::{Duration, Instant}};

/// An RPC handler, implementing the connect server API.
pub struct RpcHandler {
  socket: SocketAddrV4,
  clients: ClientManager,
  startup: Instant,
}

impl RpcHandler {
  /// Constructs a new RPC handler.
  pub fn new(socket: SocketAddrV4, clients: ClientManager) -> Self {
    RpcHandler {
      socket,
      clients,
      startup: Instant::now(),
    }
  }

  /// Returns the current uptime of the service.
  fn uptime(&self) -> Duration { Instant::now().duration_since(self.startup) }
}

impl ConnectServerApi for RpcHandler {
  fn status(&self) -> Result<ConnectServerStatus, Error> {
    Ok(ConnectServerStatus {
      host: *self.socket.ip(),
      port: self.socket.port(),
      uptime: self.uptime().as_secs(),
      clients: self.clients.len(),
    })
  }

  fn version(&self) -> Result<&'static str, Error> { Ok(env!("CARGO_PKG_VERSION")) }
}
