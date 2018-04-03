use controller::GameServerController;
use std::io;
use std::thread::{self, JoinHandle};
use tap::TapResultOps;

mod server;

/// An implementation of a Game Service.
pub struct GameService {
  controller: GameServerController,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl GameService {
  /// Spawns a new Game Service instance.
  pub fn spawn(controller: GameServerController) -> Self {
    let cc = controller.clone();
    let thread = thread::spawn(move || server::serve(cc));

    GameService {
      controller,
      thread: Some(thread),
    }
  }

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

/// Closes the service upon destruction.
impl Drop for GameService {
  fn drop(&mut self) { let _ = self.controller.close(); }
}
