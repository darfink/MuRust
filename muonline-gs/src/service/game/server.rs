use super::GameServiceControl;
use futures::IntoFuture;
use std::io;

/// Starts the Game Server using the supplied state.
pub fn serve<S, C>(_state: S, _cancel: C) -> io::Result<()>
where
  S: GameServiceControl,
  C: IntoFuture<Item = (), Error = io::Error>,
  C::Future: Send + 'static,
{
  Ok(())
}
