use context::GameServerContext;
use jsonrpc_core::Error;
use rpc::api::{GameServerApi, GameServerStatus};
use std::time::Duration;

/// An RPC handler, implementing the connect server API.
pub struct RpcHandler {
  context: GameServerContext,
}

impl RpcHandler {
  /// Constructs a new RPC handler.
  pub fn new(context: GameServerContext) -> Self { RpcHandler { context } }

  /// Returns the current uptime of the service.
  fn uptime(&self) -> Duration { Duration::new(10, 1) }
}

impl GameServerApi for RpcHandler {
  fn status(&self) -> Result<GameServerStatus, Error> {
    let socket = self.context.socket();

    // TODO: Implement duration!
    Ok(GameServerStatus {
      id: self.context.config().id,
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.uptime().as_secs(),
      clients: self.context.clients_connected(),
      max_clients: self.context.config().maximum_players,
    })
  }

  fn version(&self) -> Result<&'static str, Error> { Ok(env!("CARGO_PKG_VERSION")) }
}
