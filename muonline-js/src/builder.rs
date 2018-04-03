use service::{JoinService, QueryableGameServer, RpcService};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use {mugs, JoinServer};

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

  /// Adds a local Game Server.
  pub fn local(mut self, builder: mugs::ServerBuilder) -> Self {
    self.game_servers.push(GameServerOption::Local(builder));
    self
  }

  /// Spawns the Join & RPC services and returns a controller.
  pub fn spawn(self) -> io::Result<JoinServer> {
    let game_servers = Self::spawn_game_servers(self.game_servers)?;
    let game_servers_count = game_servers.len();

    let join_service = JoinService::spawn(self.socket_join, game_servers)?;
    let rpc_service = RpcService::spawn(self.socket_rpc, join_service.interface())?;

    info!("Running with {} Game Server(s)", game_servers_count);
    info!("RPC servicing at {}", rpc_service.uri());

    Ok(JoinServer {
      join_service,
      rpc_service,
    })
  }

  /// Returns registered servers as a Queryable collection.
  fn spawn_game_servers(
    servers: Vec<GameServerOption>,
  ) -> io::Result<Vec<Box<QueryableGameServer>>> {
    servers
      .into_iter()
      .map(|option| match option {
        // Remote server's are assumed to already have been spawned
        GameServerOption::Remote(string) => Ok(Box::new(string) as Box<QueryableGameServer>),
        // Local game servers must be spawned and managed
        GameServerOption::Local(builder) => builder
          .spawn()
          .map(|s| Box::new(s) as Box<QueryableGameServer>),
      })
      .collect::<Result<Vec<Box<QueryableGameServer>>, _>>()
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
  Local(mugs::ServerBuilder),
}
