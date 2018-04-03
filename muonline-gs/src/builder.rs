use GameServer;
use service::GameService;
use service::RpcService;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};

/// A builder for the GameServer.
pub struct ServerBuilder {
  socket_rpc: SocketAddr,
  socket_game: SocketAddrV4,
  server_id: u16,
  capacity: usize,
}

impl ServerBuilder {
  pub fn new(id: u16) -> Self {
    ServerBuilder {
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_game: "0.0.0.0:2005".parse().unwrap(),
      server_id: id,
      capacity: 100,
    }
  }

  /// Set's the Game Server capacity.
  pub fn capacity(mut self, capacity: usize) -> Self {
    self.capacity = capacity;
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
    let game_service = GameService::spawn(self.socket_game, self.server_id, self.capacity);
    let rpc_service = RpcService::spawn(self.socket_rpc, game_service.interface())?;

    Ok(GameServer {
      game_service,
      rpc_service,
    })
  }
}
