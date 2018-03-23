use std::net::SocketAddr;

/// A trait enabling communication with the server front-end.
pub trait Ipc: Send + Sync {
  /// Called whenever a client connects.
  fn on_connect(&self, _: SocketAddr) { }

  /// Called whenever a client disconnects.
  fn on_disconnect(&self, _: SocketAddr) { }

  /// Request the front-end to exit.
  fn on_exit(&self) { }
}

/// A facade for an empty IPC interface.
#[derive(Clone)]
pub struct DefaultIpc(());

impl DefaultIpc {
  /// Constructs an empty IPC interface.
  pub fn new() -> Self {
    DefaultIpc(())
  }
}

impl Ipc for DefaultIpc { }