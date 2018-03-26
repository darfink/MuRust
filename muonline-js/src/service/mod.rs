use self::serve::serve;
use futures::sync::oneshot;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Deref;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use std::{io, thread};
use util::CancellableService;

mod serve;

pub struct JoinService(CancellableService);

impl JoinService {
  /// Starts a new Join Service.
  pub fn spawn(context: Arc<JoinServiceContext>) -> Self {
    let (tx, rx) = oneshot::channel();
    let thread = thread::spawn(move || serve(context, rx));
    JoinService(CancellableService::new(thread, tx))
  }

  pub fn wait(self) -> io::Result<()> {
    self.0.wait()
  }

  pub fn close(self) -> io::Result<()> {
    self.0.close()
  }
}

#[derive(Debug)]
pub struct Client {
  id: usize,
  socket: SocketAddr,
}

impl Client {
  pub fn new(id: usize, socket: SocketAddr) -> Self {
    Client { id, socket }
  }
}

impl Deref for Client {
  type Target = SocketAddr;

  fn deref(&self) -> &Self::Target {
    &self.socket
  }
}

pub struct JoinServiceContext {
  socket: SocketAddrV4,
  startup: Instant,
  clients: Mutex<Vec<Client>>,
  id_pool: AtomicUsize,
}

impl JoinServiceContext {
  pub fn new(socket: SocketAddrV4) -> Self {
    JoinServiceContext {
      socket,
      startup: Instant::now(),
      clients: Mutex::new(Vec::new()),
      id_pool: AtomicUsize::new(1),
    }
  }

  pub fn add_client(&self, socket: SocketAddr) -> usize {
    let id = self.id_pool.fetch_add(1, Ordering::SeqCst);

    self.clients.lock().unwrap().push(Client::new(id, socket));
    id
  }

  pub fn remove_client(&self, id: usize) {
    self
      .clients
      .lock()
      .unwrap()
      .retain(|client| client.id != id);
  }

  pub fn number_of_clients(&self) -> usize {
    self.clients.lock().unwrap().len()
  }

  pub fn socket(&self) -> SocketAddrV4 {
    self.socket
  }

  pub fn uptime(&self) -> Duration {
    Instant::now().duration_since(self.startup)
  }
}
