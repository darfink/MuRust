use self::controller::JoinServiceController;
use self::traits::JoinServiceControl;
use futures::sync::mpsc;
use futures::{Future, Stream};
use service::JoinServiceInterface;
use std::io;
use std::net::SocketAddrV4;
use std::thread::{self, JoinHandle};
use tap::TapResultOps;

mod controller;
mod server;
mod traits;

pub struct JoinService {
  controller: JoinServiceController,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl JoinService {
  /// Spawns a new Join Service instance.
  pub fn spawn(socket: SocketAddrV4) -> Self {
    let (tx, rx) = mpsc::channel(1);

    let controller = JoinServiceController::new(socket, tx);
    let thread = thread::spawn(closet!([controller] move || {
      let close = rx.into_future().map(|_| ()).map_err(|_| io::ErrorKind::Other.into());
      server::serve(controller, close)
    }));

    JoinService {
      controller,
      thread: Some(thread),
    }
  }

  /// Returns an interface to the service instance.
  pub fn interface(&self) -> impl JoinServiceInterface { self.controller.clone() }

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

impl Drop for JoinService {
  fn drop(&mut self) { let _ = self.controller.close(); }
}
