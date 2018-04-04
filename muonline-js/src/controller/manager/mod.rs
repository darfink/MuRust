use self::client::Client;
use std::net::SocketAddrV4;
use std::sync::{Mutex, MutexGuard, atomic::{AtomicUsize, Ordering}};

mod client;

/// Client manager for all Join Service users.
pub struct ClientManager {
  clients: Mutex<Vec<Client>>,
  idx: AtomicUsize,
}

impl ClientManager {
  /// Constructs a new client manager.
  pub fn new() -> Self {
    ClientManager {
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

  /// Returns the inner client vector.
  fn clients<'a>(&'a self) -> MutexGuard<'a, Vec<Client>> { self.clients.lock().unwrap() }
}
