pub use self::context::GameServerContext;
use futures::{Future, Sink, sync::mpsc};
use std::io;
use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod context;

/// The internal data of a Game Server controller instance.
struct GameServerControllerInner {
  close_rx: Mutex<Option<mpsc::Receiver<()>>>,
  close_tx: mpsc::Sender<()>,
  context: GameServerContext,
  max_clients: usize,
  server_id: u16,
  socket: SocketAddrV4,
  start_time: Instant,
}

/// A Game Server controller.
#[derive(Clone)]
pub struct GameServerController(Arc<GameServerControllerInner>);

impl GameServerController {
  /// Constructs a new Game Server controller.
  pub fn new(socket: SocketAddrV4, server_id: u16, max_clients: usize, context: GameServerContext) -> Self {
    let (close_tx, close_rx) = mpsc::channel(1);
    let start_time = Instant::now();

    GameServerController(Arc::new(GameServerControllerInner {
      close_rx: Mutex::new(Some(close_rx)),
      close_tx,
      context,
      max_clients,
      server_id,
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

  /// Returns the server's id.
  pub fn id(&self) -> u16 { self.0.server_id }

  /// Returns the server's context.
  pub fn context(&self) -> &GameServerContext { &self.0.context }

  /// Returns the maximum amount of clients.
  pub fn max_clients(&self) -> usize { self.0.max_clients }

  /// Returns the join service's socket.
  pub fn socket(&self) -> SocketAddrV4 { self.0.socket }

  /// Returns the server's uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.0.start_time) }
}
