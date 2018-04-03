use controller::JoinServerController;
use std::io;
use std::thread::{self, JoinHandle};
use tap::TapResultOps;

mod server;

/// An implementation of a Join Service.
pub struct JoinService {
  controller: JoinServerController,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl JoinService {
  /// Spawns a new Join Service instance.
  pub fn spawn(controller: JoinServerController) -> Self {
    let cc = controller.clone();
    let thread = thread::spawn(move || server::serve(cc));

    JoinService {
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
      .tap_err(|any| debug!("<JoinService> {:#?}", any))
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread panicked"))
      .and_then(|result| result)
  }
}

/// Closes the service upon destruction.
impl Drop for JoinService {
  fn drop(&mut self) { let _ = self.controller.close(); }
}
