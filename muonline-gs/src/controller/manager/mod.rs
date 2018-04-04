use self::client::Client;
use std::net::SocketAddrV4;
use std::sync::{Mutex, MutexGuard, atomic::{AtomicUsize, Ordering}};

mod client;

/// Client manager for all Game Service users.
pub struct ClientManager {
  capacity: usize,
  clients: Mutex<Vec<Client>>,
  idx: AtomicUsize,
}

impl ClientManager {
  /// Constructs a new server context.
  pub fn new(max_clients: usize) -> Self {
    ClientManager {
      capacity: max_clients,
      clients: Mutex::new(Vec::new()),
      idx: AtomicUsize::new(0),
    }
  }

  /// Adds a new client.
  pub fn add(&self, socket: SocketAddrV4) -> usize {
    let id = self.idx.fetch_add(1, Ordering::Relaxed);
    self.clients().push(Client::new(id, socket));
    id
  }

  /// Removes a client.
  pub fn remove(&self, id: usize) { self.clients().retain(|c| c.id != id); }

  /// Returns the number of clients.
  pub fn len(&self) -> usize { self.clients().len() }

  /// Returns the maximum number of clients.
  pub fn capacity(&self) -> usize { self.capacity }

  /// Returns the inner client vector.
  fn clients<'a>(&'a self) -> MutexGuard<'a, Vec<Client>> { self.clients.lock().unwrap() }
}
