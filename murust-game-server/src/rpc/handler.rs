use {ClientManager, ServerInfo};
use jsonrpc_core::Error;
use rpc::api::{GameServerApi, GameServerStatus};
use std::time::{Duration, Instant};

/// An RPC handler, implementing the connect server API.
pub struct RpcHandler {
  server_info: ServerInfo,
  clients: ClientManager,
  startup: Instant,
}

impl RpcHandler {
  /// Constructs a new RPC handler.
  pub fn new(server_info: ServerInfo, clients: ClientManager) -> Self {
    RpcHandler {
      server_info,
      clients,
      startup: Instant::now(),
    }
  }

  /// Returns the current uptime of the service.
  fn uptime(&self) -> Duration { Instant::now().duration_since(self.startup) }
}

impl GameServerApi for RpcHandler {
  fn status(&self) -> Result<GameServerStatus, Error> {
    let socket = self.server_info.socket();

    Ok(GameServerStatus {
      id: self.server_info.id(),
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.uptime().as_secs(),
      clients: self.clients.len(),
      max_clients: self.clients.capacity(),
    })
  }

  fn version(&self) -> Result<&'static str, Error> { Ok(env!("CARGO_PKG_VERSION")) }
}
