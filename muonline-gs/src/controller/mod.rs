pub use self::manager::ClientManager;
use futures::{Future, Sink, sync::mpsc};
use std::io;
use std::net::SocketAddrV4;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod manager;

/// A Game Server controller.
#[derive(Clone)]
pub struct GameServerController(Arc<GameServerControllerInner>);

impl GameServerController {
  /// Constructs a new Game Server controller.
  pub fn new(socket: SocketAddrV4, server_id: u16, client_manager: ClientManager) -> Self {
    let (close_tx, close_rx) = mpsc::channel(1);
    let inner = GameServerControllerInner {
      boot_time: Instant::now(),
      client_manager,
      close_rx: Mutex::new(Some(close_rx)),
      close_tx,
      server_id,
      server_socket: Mutex::new(socket),
    };

    GameServerController(Arc::new(inner))
  }
}

impl Deref for GameServerController {
  type Target = GameServerControllerInner;

  /// Returns the inner context.
  fn deref(&self) -> &Self::Target { &*self.0 }
}

/// The internal data of a Game Server controller instance.
pub struct GameServerControllerInner {
  boot_time: Instant,
  client_manager: ClientManager,
  close_rx: Mutex<Option<mpsc::Receiver<()>>>,
  close_tx: mpsc::Sender<()>,
  server_id: u16,
  server_socket: Mutex<SocketAddrV4>,
}

impl GameServerControllerInner {
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

  /// Takes the close receiver out of the controller.
  pub fn take_close_receiver(&self) -> mpsc::Receiver<()> {
    self
      .close_rx
      .lock()
      .unwrap()
      .take()
      .expect("retrieving empty receiver")
  }

  /// Returns the server's id.
  pub fn id(&self) -> u16 { self.server_id }

  /// Returns the game service's socket.
  pub fn socket(&self) -> SocketAddrV4 { *self.server_socket.lock().unwrap() }

  /// Refreshes the game service's socket.
  pub fn refresh_socket(&self, socket: SocketAddrV4) {
    *self.server_socket.lock().unwrap() = socket;
  }

  /// Returns the server's context.
  pub fn client_manager(&self) -> &ClientManager { &self.client_manager }

  /// Returns the server's uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.boot_time) }
}
