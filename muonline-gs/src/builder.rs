use GameServer;
use controller::{GameServerContext, GameServerController};
use service::{GameService, RpcService};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};

/// A builder for the GameServer.
pub struct ServerBuilder {
  socket_rpc: SocketAddr,
  socket_game: SocketAddrV4,
  server_id: u16,
  max_clients: usize,
}

impl ServerBuilder {
  pub fn new(id: u16) -> Self {
    ServerBuilder {
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_game: "0.0.0.0:2005".parse().unwrap(),
      server_id: id,
      max_clients: 100,
    }
  }

  /// Set's the Game Server capacity.
  pub fn max_clients(mut self, max_clients: usize) -> Self {
    self.max_clients = max_clients;
    self
  }

  /// Set's the GameService socket.
  pub fn game(mut self, socket: SocketAddrV4) -> Self {
    self.socket_game = socket;
    self
  }

  /// Set's the RPC Service socket.
  pub fn rpc(mut self, socket: SocketAddr) -> Self {
    self.socket_rpc = socket;
    self
  }

  /// Spawns the Game & RPC services and returns a controller.
  pub fn spawn(self) -> io::Result<GameServer> {
    let context = GameServerContext::new();
    let controller =
      GameServerController::new(self.socket_game, self.server_id, self.max_clients, context);

    let game_service = GameService::spawn(controller.clone());
    let rpc_service = RpcService::spawn(self.socket_rpc, controller)?;

    info!("RPC servicing at {}", rpc_service.uri());

    Ok(GameServer {
      game_service,
      rpc_service,
    })
  }
}
