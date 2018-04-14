use game::GameService;
use murust_service::ServiceManager;
use rpc::{RpcHandler, RpcService};
use std::{io, net::{SocketAddr, SocketAddrV4}};
use {GameServerId, ServerInfo, ClientManager, GameServer};

/// A builder for the Game Server.
pub struct ServerBuilder {
  max_clients: usize,
  server_id: GameServerId,
  service_manager: ServiceManager,
  socket_game: SocketAddrV4,
  socket_rpc: SocketAddr,
}

impl ServerBuilder {
  /// Constructs a new server builder.
  pub fn new(server_id: GameServerId, service_manager: ServiceManager) -> Self {
    ServerBuilder {
      max_clients: 100,
      server_id,
      service_manager,
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_game: "0.0.0.0:0".parse().unwrap(),
    }
  }

  /// Set's the gmae server ID.
  pub fn server_id(mut self, id: GameServerId) -> Self {
    self.server_id = id;
    self
  }

  /// Set's the Game Service socket.
  pub fn socket_game(mut self, socket: SocketAddrV4) -> Self {
    self.socket_game = socket;
    self
  }

  /// Set's the RPC Service socket.
  pub fn socket_rpc(mut self, socket: SocketAddr) -> Self {
    self.socket_rpc = socket;
    self
  }

  /// Set's the max number of clients.
  pub fn max_clients(mut self, clients: usize) -> Self {
    self.max_clients = clients;
    self
  }

  /// Spawns the Game & RPC services and returns a controller.
  pub fn spawn(self) -> io::Result<GameServer> {
    let client_manager = ClientManager::new(self.max_clients);
    let server_info = ServerInfo::new(self.server_id, self.socket_game);

    client_manager.add_disconnect_listener(|socket| info!("Client<{}> disconnected", socket.ip()));
    client_manager.add_connect_listener(|socket| {
      info!("Client<{}> connected", socket.ip());
      true
    });

    let game_service = GameService::spawn(
      server_info.clone(),
      self.service_manager,
      client_manager.clone(),
    );

    let rpc_service = RpcService::spawn(
      self.socket_rpc,
      RpcHandler::new(server_info, client_manager.clone()),
    )?;

    Ok(GameServer {
      client_manager,
      game_service,
      rpc_service,
    })
  }
}
