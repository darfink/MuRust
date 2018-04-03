pub use self::{browser::GameServerBrowser, context::JoinServerContext};
use futures::{Future, Sink, sync::mpsc};
use std::io;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod browser;
mod context;

/// The internal data of a Join Server controller instance.
struct JoinServerControllerInner {
  browser: GameServerBrowser,
  close_rx: Mutex<Option<mpsc::Receiver<()>>>,
  close_tx: mpsc::Sender<()>,
  context: JoinServerContext,
  socket: SocketAddrV4,
  start_time: Instant,
}

/// A Join Server controller.
#[derive(Clone)]
pub struct JoinServerController(Arc<JoinServerControllerInner>);

impl JoinServerController {
  /// Constructs a new Join Server controller.
  pub fn new(socket: SocketAddrV4, context: JoinServerContext, browser: GameServerBrowser) -> Self {
    let (close_tx, close_rx) = mpsc::channel(1);
    let start_time = Instant::now();

    JoinServerController(Arc::new(JoinServerControllerInner {
      browser,
      close_rx: Mutex::new(Some(close_rx)),
      close_tx,
      context,
      socket,
      start_time,
    }))
  }

  /// Signals the server to close and finish its operations.
  pub fn close(&self) -> io::Result<()> {
    let inner = &*self.0;
    inner
      .close_tx
      .clone()
      .send(())
      .wait()
      .map(|_| ())
      .map_err(|_| io::ErrorKind::BrokenPipe.into())
  }

  /// Takes the close receiver out of the controller.
  pub fn take_close_receiver(&self) -> mpsc::Receiver<()> {
    let inner = &*self.0;
    inner
      .close_rx
      .lock()
      .unwrap()
      .take()
      .expect("retrieving empty receiver")
  }

  /// Returns the game server browser.
  pub fn browser(&self) -> &GameServerBrowser { &self.0.browser }

  /// Returns the server's context.
  pub fn context(&self) -> &JoinServerContext { &self.0.context }

  /// Returns the join service's socket.
  pub fn socket(&self) -> SocketAddrV4 { self.0.socket }

  /// Returns the server's uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.0.start_time) }
}
