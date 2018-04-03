use self::client::Client;
use super::{GameServerBrowser, JoinServiceControl};
use futures::{Future, Sink, Stream, sync::mpsc};
use mugs;
use service::{JoinServiceInterface, JoinServiceListen};
use std::io;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};

mod client;

/// Internal data of the Join Service controller.
struct JoinServiceContext {
  client_idx: AtomicUsize,
  clients: Mutex<Vec<Client>>,
  browser: GameServerBrowser,
  socket: SocketAddrV4,
  start_time: Instant,
}

/// A Join Service controller instance.
#[derive(Clone)]
pub struct JoinServiceController {
  close_tx: mpsc::Sender<()>,
  context: Arc<JoinServiceContext>,
}

impl JoinServiceController {
  /// Constructs a new Join Service controller.
  pub fn new(socket: SocketAddrV4, browser: GameServerBrowser, close_tx: mpsc::Sender<()>) -> Self {
    JoinServiceController {
      close_tx,
      context: Arc::new(JoinServiceContext {
        client_idx: AtomicUsize::new(0),
        clients: Mutex::new(Vec::new()),
        browser,
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

impl JoinServiceListen for JoinServiceController {
  /// Returns the service's socket.
  fn socket(&self) -> SocketAddrV4 { self.context.socket }
}

impl JoinServiceInterface for JoinServiceController {
  /// Returns the number of clients.
  fn number_of_clients(&self) -> usize { self.context.clients.lock().unwrap().len() }

  /// Returns the current uptime.
  fn uptime(&self) -> Duration { Instant::now().duration_since(self.context.start_time) }
}

impl JoinServiceControl for JoinServiceController {
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

  /// Queries all available game servers.
  fn query_game_servers(
    &self,
  ) -> Box<Stream<Item = mugs::rpc::GameServerStatus, Error = io::Error> + Send> {
    Box::new(self.context.browser.query_servers())
  }
}
