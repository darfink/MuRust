use futures::{Future, Sink, sync::mpsc};
use game::server;
use murust_service::ServiceManager;
use std::{io, thread::{self, JoinHandle}};
use tap::TapResultOps;
use {ClientManager, ServerInfo};

/// Wraps the underlying game server thread.
pub struct GameService {
  close_tx: mpsc::Sender<()>,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl GameService {
  /// Spawns a new Game Service instance.
  pub fn spawn(
    server_info: ServerInfo,
    service_manager: ServiceManager,
    client_manager: ClientManager,
  ) -> Self {
    let (close_tx, close_rx) = mpsc::channel(1);
    let thread =
      thread::spawn(move || server::serve(server_info, service_manager, client_manager, close_rx));

    GameService {
      close_tx,
      thread: Some(thread),
    }
  }

  /// Stops the service.
  pub fn stop(mut self) -> io::Result<()> {
    self.close_server()?;
    Self::join_thread(self.thread.take().unwrap())
  }

  /// Will block, waiting for the service to finish.
  pub fn wait(mut self) -> io::Result<()> { Self::join_thread(self.thread.take().unwrap()) }

  /// Sends a close message to the server.
  fn close_server(&self) -> io::Result<()> {
    self
      .close_tx
      .clone()
      .send(())
      .wait()
      .map(|_| ())
      .map_err(|_| io::ErrorKind::BrokenPipe.into())
  }

  /// Joins the server thread with the current thread.
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
  fn drop(&mut self) { let _ = self.close_server(); }
}
