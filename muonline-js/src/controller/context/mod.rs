use self::client::Client;
use std::net::SocketAddrV4;
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};

mod client;

/// Internal data of the Join Service controller.
pub struct JoinServerContext {
  client_idx: AtomicUsize,
  clients: Mutex<Vec<Client>>,
}

impl JoinServerContext {
  /// Constructs a new server context.
  pub fn new() -> Self {
    JoinServerContext {
      client_idx: AtomicUsize::new(0),
      clients: Mutex::new(Vec::new()),
    }
  }

  /// Adds a new client to the state.
  pub fn add_client(&self, socket: SocketAddrV4) -> usize {
    let id = self.client_idx.fetch_add(1, Ordering::Relaxed);
    self.clients.lock().unwrap().push(Client::new(id, socket));
    id
  }

  /// Removes a client from the state.
  pub fn remove_client(&self, id: usize) { self.clients.lock().unwrap().retain(|c| c.id != id); }

  /// Returns the number of clients.
  pub fn number_of_clients(&self) -> usize { self.clients.lock().unwrap().len() }
}
