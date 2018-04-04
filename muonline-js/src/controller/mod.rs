pub use self::{browser::GameServerBrowser, manager::ClientManager};
use futures::{Future, Sink, sync::mpsc};
use std::io;
use std::net::SocketAddrV4;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod browser;
mod manager;

/// A Join Server controller.
#[derive(Clone)]
pub struct JoinServerController(Arc<JoinServerControllerInner>);

impl JoinServerController {
  /// Constructs a new Join Server controller.
  pub fn new(socket_join: SocketAddrV4, server_browser: GameServerBrowser) -> Self {
    let (js_close_tx, js_close_rx) = mpsc::channel(1);
    JoinServerController(Arc::new(JoinServerControllerInner {
      boot_time: Instant::now(),
      client_manager: ClientManager::new(),
      js_close_rx: Mutex::new(Some(js_close_rx)),
      js_close_tx,
      server_browser,
      socket_join,
    }))
  }
}

impl Deref for JoinServerController {
  type Target = JoinServerControllerInner;

  /// Returns the inner context.
  fn deref(&self) -> &Self::Target { &*self.0 }
}

/// The internal data of a Join Server controller instance.
pub struct JoinServerControllerInner {
  boot_time: Instant,
  client_manager: ClientManager,
  js_close_rx: Mutex<Option<mpsc::Receiver<()>>>,
  js_close_tx: mpsc::Sender<()>,
  server_browser: GameServerBrowser,
  socket_join: SocketAddrV4,
}

impl JoinServerControllerInner {
  /// Returns the game server browser.
  pub fn server_browser(&self) -> &GameServerBrowser { &self.server_browser }

  /// Returns the server's context.
  pub fn client_manager(&self) -> &ClientManager { &self.client_manager }

  /// Returns the join service's socket.
  pub fn socket(&self) -> SocketAddrV4 { self.socket_join }

  /// Returns the server's uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.boot_time) }

  /// Signals the server to close and finish its operations.
  pub fn close(&self) -> io::Result<()> {
    self
      .js_close_tx
      .clone()
      .send(())
      .wait()
      .map(|_| ())
      .map_err(|_| io::ErrorKind::BrokenPipe.into())
  }

  /// Takes the close receiver out of the controller.
  pub fn take_close_receiver(&self) -> mpsc::Receiver<()> {
    self
      .js_close_rx
      .lock()
      .unwrap()
      .take()
      .expect("retrieving empty receiver")
  }
}
