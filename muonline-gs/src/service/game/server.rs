use controller::GameServerController;
use std::io;

/// Starts the Game Server using the supplied controller.
pub fn serve(_controller: GameServerController) -> io::Result<()> {
  Ok(())
}
