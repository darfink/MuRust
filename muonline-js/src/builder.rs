use JoinServer;
use service::JoinService;
use service::RpcService;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};

/// A builder for the Join Server.
pub struct ServerBuilder {
  socket_rpc: SocketAddr,
  socket_join: SocketAddrV4,
  game_servers: Vec<GameServerOption>,
}

impl ServerBuilder {
  /// Set's the Join Service socket.
  pub fn join(mut self, socket: SocketAddrV4) -> Self {
    self.socket_join = socket;
    self
  }

  /// Set's the RPC Service socket.
  pub fn rpc(mut self, socket: SocketAddr) -> Self {
    self.socket_rpc = socket;
    self
  }

  /// Adds a remote Game Server.
  pub fn remote(mut self, uri: String) -> Self {
    self.game_servers.push(GameServerOption::Remote(uri));
    self
  }

  // Adds a local Game Server.
  // pub fn local(mut self, builder: mugs::ServerBuilder) -> Self {
  //   self.game_servers.push(GameServerOption::Local(builder));
  //   self
  // }

  /// Spawns the Join & RPC services and returns a controller.
  pub fn spawn(self) -> io::Result<JoinServer> {
    let join_service = JoinService::spawn(self.socket_join /* , self.game_servers */);
    let rpc_service = RpcService::spawn(self.socket_rpc, join_service.interface())?;

    Ok(JoinServer {
      join_service,
      rpc_service,
    })
  }
}

impl Default for ServerBuilder {
  fn default() -> Self {
    ServerBuilder {
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_join: "0.0.0.0:2004".parse().unwrap(),
      game_servers: Vec::new(),
    }
  }
}

enum GameServerOption {
  Remote(String),
  // Local(mugs::ServerBuilder),
}
