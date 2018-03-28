use self::worker::JoinServiceWorker;
use super::JoinServiceInterface;
use futures::sync::mpsc;
use std::io;
use std::net::SocketAddrV4;
use std::thread::{self, JoinHandle};
use tap::TapResultOps;

mod context;
mod worker;

pub struct JoinService {
  worker: JoinServiceWorker,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl JoinService {
  /// Spawns a new Join Service instance.
  pub fn spawn(socket: SocketAddrV4) -> Self {
    let (tx, rx) = mpsc::channel(1);
    let worker = JoinServiceWorker::new(socket, tx);
    let thread = thread::spawn(clone_army!([worker] || worker.serve(rx)));

    JoinService {
      worker,
      thread: Some(thread),
    }
  }

  /// Returns an interface to the service instance.
  pub fn interface(&self) -> impl JoinServiceInterface { self.worker.clone() }

  /// Closes the service.
  pub fn close(mut self) -> io::Result<()> {
    self.worker.close()?;
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
  fn drop(&mut self) { let _ = self.worker.close(); }
}
