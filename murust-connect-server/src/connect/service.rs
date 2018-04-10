use connect::server;
use futures::{Future, Sink, sync::mpsc};
use std::{io, net::SocketAddrV4, thread::{self, JoinHandle}};
use tap::TapResultOps;
use {ClientManager, GameServerBrowser};

/// Wraps the underlying connect server thread.
pub struct ConnectService {
  close_tx: mpsc::Sender<()>,
  thread: Option<JoinHandle<io::Result<()>>>,
}

impl ConnectService {
  /// Spawns a new Connect Service instance.
  pub fn spawn(
    socket: SocketAddrV4,
    server_browser: GameServerBrowser,
    client_manager: ClientManager,
  ) -> Self {
    let (close_tx, close_rx) = mpsc::channel(1);
    let thread =
      thread::spawn(move || server::serve(socket, server_browser, client_manager, close_rx));

    ConnectService {
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
      .tap_err(|any| debug!("<ConnectService> {:#?}", any))
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread panicked"))
      .and_then(|result| result)
  }
}

/// Closes the service upon destruction.
impl Drop for ConnectService {
  fn drop(&mut self) { let _ = self.close_server(); }
}
