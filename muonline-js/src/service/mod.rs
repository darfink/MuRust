use self::serve::serve;
use futures::sync::oneshot;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Deref;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

mod serve;

pub struct JoinService {
  thread: JoinHandle<io::Result<()>>,
  cancel: oneshot::Sender<()>,
}

impl JoinService {
  /// Starts a new Join Service.
  pub fn spawn(context: Arc<JoinServiceContext>) -> Self {
    let (tx, rx) = oneshot::channel();
    let thread = thread::spawn(move || serve(context, rx));

    JoinService { thread, cancel: tx }
  }

  pub fn wait(self) -> io::Result<()> {
    Self::join_thread(self.thread)
  }

  pub fn close(self) -> io::Result<()> {
    self
      .cancel
      .send(())
      .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "service already closed"))?;
    Self::join_thread(self.thread)
  }

  fn join_thread(thread: JoinHandle<io::Result<()>>) -> io::Result<()> {
    thread
      .join()
      .map_err(|any| {
        let error = any.downcast_ref::<io::Error>().unwrap();
        io::Error::new(error.kind(), error.to_string())
      })
      .and_then(|r| r)
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
