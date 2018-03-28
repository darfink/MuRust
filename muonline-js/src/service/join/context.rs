use std::net::SocketAddrV4;
use std::ops::Deref;
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};

/// Context for the inner Join Service.
pub struct JoinServiceContext {
  client_idx: AtomicUsize,
  clients: Mutex<Vec<Client>>,
  socket: SocketAddrV4,
  start_time: Instant,
}

impl JoinServiceContext {
  /// Constructs a new Join Service context.
  pub fn new(socket: SocketAddrV4) -> Self {
    JoinServiceContext {
      client_idx: AtomicUsize::new(0),
      clients: Mutex::new(Vec::new()),
      socket,
      start_time: Instant::now(),
    }
  }

  /// Adds a new client to the context.
  pub fn add_client(&self, socket: SocketAddrV4) -> usize {
    let id = self.client_idx.fetch_add(1, Ordering::Relaxed);
    self.clients.lock().unwrap().push(Client::new(id, socket));
    id
  }

  /// Removes a client from the context.
  pub fn remove_client(&self, id: usize) { self.clients.lock().unwrap().retain(|c| c.id != id); }

  /// Returns the number of clients.
  pub fn number_of_clients(&self) -> usize { self.clients.lock().unwrap().len() }

  /// Returns the service's socket.
  pub fn socket(&self) -> SocketAddrV4 { self.socket }

  /// Returns the service's current uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.start_time) }
}

/// A representation of a Join Service client.
#[derive(Debug)]
struct Client {
  id: usize,
  socket: SocketAddrV4,
}

impl Client {
  /// Constructs a new client instance.
  fn new(id: usize, socket: SocketAddrV4) -> Self { Client { id, socket } }
}

impl Deref for Client {
  type Target = SocketAddrV4;

  /// Returns a reference to the client's socket.
  fn deref(&self) -> &Self::Target { &self.socket }
}
