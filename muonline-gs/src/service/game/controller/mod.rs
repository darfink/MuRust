use self::client::Client;
use super::GameServiceControl;
use futures::{Future, Sink, sync::mpsc};
use service::{GameServiceInterface, GameServiceListen};
use std::io;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};

mod client;

/// Internal data of the Game Service controller.
struct GameServiceContext {
  client_idx: AtomicUsize,
  clients: Mutex<Vec<Client>>,
  server_capacity: usize,
  server_id: u16,
  socket: SocketAddrV4,
  start_time: Instant,
}

/// A Game Service controller instance.
#[derive(Clone)]
pub struct GameServiceController {
  close_tx: mpsc::Sender<()>,
  context: Arc<GameServiceContext>,
}

impl GameServiceController {
  /// Constructs a new Game Service controller.
  pub fn new(socket: SocketAddrV4, id: u16, capacity: usize, close_tx: mpsc::Sender<()>) -> Self {
    GameServiceController {
      close_tx,
      context: Arc::new(GameServiceContext {
        client_idx: AtomicUsize::new(0),
        clients: Mutex::new(Vec::new()),
        server_capacity: capacity,
        server_id: id,
        socket,
        start_time: Instant::now(),
      }),
    }
  }

  /// Signals the server to close and finish its operations.
  pub fn close(&self) -> io::Result<()> {
    self
      .close_tx
      .clone()
      .send(())
      .wait()
      .map(|_| ())
      .map_err(|_| io::ErrorKind::BrokenPipe.into())
  }
}

impl GameServiceListen for GameServiceController {
  /// Returns the service's socket.
  fn socket(&self) -> SocketAddrV4 { self.context.socket }
}

impl GameServiceInterface for GameServiceController {
  /// Returns the server's id.
  fn id(&self) -> u16 { self.context.server_id }

  /// Returns the server's capacity.
  fn capacity(&self) -> usize { self.context.server_capacity }

  /// Returns the number of clients.
  fn number_of_clients(&self) -> usize { self.context.clients.lock().unwrap().len() }

  /// Returns the current uptime.
  fn uptime(&self) -> Duration { Instant::now().duration_since(self.context.start_time) }
}

impl GameServiceControl for GameServiceController {
  /// Adds a new client to the state.
  fn add_client(&self, socket: SocketAddrV4) -> usize {
    let id = self.context.client_idx.fetch_add(1, Ordering::Relaxed);
    self
      .context
      .clients
      .lock()
      .unwrap()
      .push(Client::new(id, socket));
    id
  }

  /// Removes a client from the state.
  fn remove_client(&self, id: usize) {
    self.context.clients.lock().unwrap().retain(|c| c.id != id);
  }
}
