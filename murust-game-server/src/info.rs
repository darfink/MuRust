use GameServerId;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};

/// The game server info.
#[derive(Clone)]
pub struct ServerInfo {
  server_id: GameServerId,
  socket: Arc<Mutex<SocketAddrV4>>,
}

impl ServerInfo {
  /// Constructs a new server info instance.
  pub fn new(server_id: GameServerId, socket: SocketAddrV4) -> Self {
    ServerInfo {
      server_id,
      socket: Arc::new(Mutex::new(socket)),
    }
  }

  /// Returns the ID of the server.
  pub fn id(&self) -> GameServerId { self.server_id }

  /// Returns the socket used by the server.
  pub fn socket(&self) -> SocketAddrV4 { *self.socket.lock().unwrap() }

  /// Updates the socket information of the server.
  pub fn refresh_socket(&self, socket: SocketAddrV4) { *self.socket.lock().unwrap() = socket; }
}
