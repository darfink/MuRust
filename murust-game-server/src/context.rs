use GameServerConfig;
use handlers::{self, PacketHandlerCore};
use murust_data_model::types::ObjectId;
use murust_service::ServiceManager;
use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex, MutexGuard};

/// The inner game server context.
struct InnerContext {
  clients_idx: ObjectId,
  clients: HashMap<ObjectId, SocketAddrV4>,
  socket: SocketAddrV4,
}

/// A game server context.
#[derive(Clone)]
pub struct GameServerContext {
  config: GameServerConfig,
  services: ServiceManager,
  handler: Arc<PacketHandlerCore>,
  inner: Arc<Mutex<InnerContext>>,
}

impl GameServerContext {
  /// Constructs a new server context.
  pub fn new(config: GameServerConfig, services: ServiceManager) -> Self {
    let socket = config.socket;
    let handler = Arc::new(handlers::default(&services));
    GameServerContext {
      config,
      services,
      handler,
      inner: Arc::new(Mutex::new(InnerContext {
        socket,
        clients: HashMap::new(),
        clients_idx: 0,
      })),
    }
  }

  /// Adds a new client.
  pub fn add_client(&self, socket: SocketAddrV4) -> Option<ObjectId> {
    let mut inner = self.inner();

    // TODO: Use an ID pool for this.
    if inner.clients.len() < self.config.maximum_players {
      inner.clients_idx += 1;
      let id = inner.clients_idx;
      inner.clients.insert(id, socket);
      Some(id)
    } else {
      None
    }
  }

  /// Removes a client.
  pub fn remove_client(&self, id: ObjectId) { self.inner().clients.remove(&id); }

  /// Returns the number of clients connected.
  pub fn clients_connected(&self) -> usize { self.inner().clients.len() }

  /// Returns the service manager.
  pub fn services(&self) -> &ServiceManager { &self.services }

  /// Returns the packet handler.
  pub fn packet_handler(&self) -> Arc<PacketHandlerCore> { self.handler.clone() }

  /// Returns the server config.
  pub fn config(&self) -> &GameServerConfig { &self.config }

  /// Returns the socket used by the server.
  pub fn socket(&self) -> SocketAddrV4 { self.inner().socket }

  /// Updates the socket with the address actually bound by the server.
  pub(crate) fn refresh_socket(&self, socket: SocketAddrV4) { self.inner().socket = socket; }

  /// Returns the inner context.
  fn inner(&self) -> MutexGuard<InnerContext> {
    self.inner.lock().expect("locking inner server context")
  }
}
