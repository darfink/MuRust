use self::handler::RpcHandler;
pub use self::service::RpcService;
use context::GameServerContext;
use error::Result;
use std::net::SocketAddr;

pub mod api;
mod handler;
mod service;

/// Spawns a new RPC service for the game server.
pub fn spawn_service(
  socket: SocketAddr,
  context: GameServerContext,
) -> Result<service::RpcService> {
  RpcService::spawn(socket, RpcHandler::new(context))
}
