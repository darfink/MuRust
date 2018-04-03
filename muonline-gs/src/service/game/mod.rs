use self::controller::GameServiceController;
use self::traits::GameServiceControl;
use futures::sync::mpsc;
use futures::{Future, Stream};
use service::GameServiceInterface;
use std::io;
use std::net::SocketAddrV4;
use std::thread::{self, JoinHandle};
use tap::TapResultOps;

mod controller;
mod server;
mod traits;

pub struct GameService {
  controller: GameServiceController,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl GameService {
  /// Spawns a new Game Service instance.
  pub fn spawn(socket: SocketAddrV4, id: u16, capacity: usize) -> Self {
    let (tx, rx) = mpsc::channel(1);

    let controller = GameServiceController::new(socket, id, capacity, tx);
    let thread = thread::spawn(closet!([controller] move || {
      let close = rx.into_future().map(|_| ()).map_err(|_| io::ErrorKind::Other.into());
      server::serve(controller, close)
    }));

    GameService {
      controller,
      thread: Some(thread),
    }
  }

  /// Returns an interface to the controller instance.
  pub fn interface(&self) -> impl GameServiceInterface { self.controller.clone() }

  /// Closes the service.
  pub fn close(mut self) -> io::Result<()> {
    self.controller.close()?;
    Self::join_thread(self.thread.take().unwrap())
  }

  /// Will block, waiting for the service to finish.
  pub fn wait(mut self) -> io::Result<()> { Self::join_thread(self.thread.take().unwrap()) }

  fn join_thread(thread: JoinHandle<io::Result<()>>) -> io::Result<()> {
    thread
      .join()
      .tap_err(|any| debug!("<GameService> {:#?}", any))
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread panicked"))
      .and_then(|result| result)
  }
}

impl Drop for GameService {
  /// Closes the service upon destruction.
  fn drop(&mut self) { let _ = self.controller.close(); }
}
