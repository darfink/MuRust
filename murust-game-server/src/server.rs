use context::GameServerContext;
use error::Result;
use failure::Context;
use futures::{Future, Sink, sync::mpsc};
use murust_service::ServiceManager;
use std::{thread::{self, JoinHandle}, time::{Duration, Instant}};
use tap::TapResultOps;
use {listener, GameServerConfig};

/// An implementation of a Game Server.
pub struct GameServer {
  context: GameServerContext,
  listener_close: mpsc::Sender<()>,
  listener_thread: Option<JoinHandle<Result<()>>>,
  start_time: Instant,
}

impl GameServer {
  /// Spawns a new Game Server instance.
  pub fn spawn(config: GameServerConfig, service_manager: ServiceManager) -> Self {
    let (listener_close, close_rx) = mpsc::channel(1);
    let context = GameServerContext::new(config, service_manager);
    let thread = thread::spawn(closet!([context] move || listener::listen(context, close_rx)));

    GameServer {
      context,
      listener_close,
      listener_thread: Some(thread),
      start_time: Instant::now(),
    }
  }

  /// Returns the server's current uptime.
  pub fn uptime(&self) -> Duration { Instant::now().duration_since(self.start_time) }

  /// Returns the server's context.
  pub fn context(&self) -> GameServerContext { self.context.clone() }

  /// Stops the server.
  pub fn stop(self) -> Result<()> {
    self.stop_listener()?;
    self.join_listener_thread()
  }

  /// Will block, waiting for the server to finish.
  pub fn wait(self) -> Result<()> { self.join_listener_thread() }

  /// Sends a close message to the listener thread.
  fn stop_listener(&self) -> Result<()> {
    self
      .listener_close
      .clone()
      .send(())
      .wait()
      .map(|_| ())
      .map_err(|_| Context::new("Server exit sender endpoint closed abruptly").into())
  }

  /// Joins the server thread with the current thread.
  fn join_listener_thread(mut self) -> Result<()> {
    self
      .listener_thread
      .take()
      .expect("extracting game server thread handle")
      .join()
      .tap_err(|any| debug!("{:#?}", any))
      .map_err(|_| Context::new("Listener server thread panicked").into())
      .and_then(|result| result)
  }
}

/// Closes the server upon destruction.
impl Drop for GameServer {
  fn drop(&mut self) { let _ = self.stop_listener(); }
}
