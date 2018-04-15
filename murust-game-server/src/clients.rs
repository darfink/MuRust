use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex, MutexGuard};

/// The inner contents of a client manager.
struct ClientManagerInner {
  idx: usize,
  clients: HashMap<usize, SocketAddrV4>,
  // TODO: Actually utilize this, and outside Mutex?
  #[allow(unused)]
  max_clients: usize,
  on_client_connect: Vec<Box<FnMut(SocketAddrV4) -> bool + Send>>,
  on_client_disconnect: Vec<Box<FnMut(SocketAddrV4) + Send>>,
}

/// A client manager implementation.
#[derive(Clone)]
pub struct ClientManager(Arc<Mutex<ClientManagerInner>>);

impl ClientManager {
  /// Constructs a new client manager.
  pub fn new(max_clients: usize) -> Self {
    ClientManager(Arc::new(Mutex::new(ClientManagerInner {
      on_client_connect: Vec::new(),
      on_client_disconnect: Vec::new(),
      max_clients,
      clients: HashMap::new(),
      idx: 0,
    })))
  }

  /// Adds a client connect listener.
  pub fn add_connect_listener<F: FnMut(SocketAddrV4) -> bool + Send + 'static>(&self, listener: F) {
    self.inner().on_client_connect.push(Box::new(listener));
  }

  /// Adds a client disconnect listener.
  pub fn add_disconnect_listener<F: FnMut(SocketAddrV4) + Send + 'static>(&self, listener: F) {
    self.inner().on_client_disconnect.push(Box::new(listener));
  }

  /// Adds a new client.
  pub fn add(&self, socket: SocketAddrV4) -> Option<usize> {
    let mut this = self.inner();
    if this
      .on_client_connect
      .iter_mut()
      .all(|listener| listener(socket))
    {
      this.idx += 1;
      let id = this.idx;

      this.clients.insert(id, socket);
      Some(id)
    } else {
      None
    }
  }

  /// Removes a client.
  pub fn remove(&self, id: usize) {
    let mut this = self.inner();
    this.clients.remove(&id).map(|socket| {
      this
        .on_client_disconnect
        .iter_mut()
        .for_each(|listener| listener(socket))
    });
  }

  /// Returns the maximum number of clients.
  pub fn capacity(&self) -> usize { self.inner().max_clients }

  /// Returns the number of clients.
  pub fn len(&self) -> usize { self.inner().clients.len() }

  /// Returns the inner context.
  fn inner(&self) -> MutexGuard<ClientManagerInner> {
    self.0.lock().expect("locking inner client manager context")
  }
}
