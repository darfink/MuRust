use error::{cxerr, Result};
use player::{Player, PlayerState};

pub struct CharacterSelectAction;

impl CharacterSelectAction {
  pub fn select(&self, player: &mut Player, name: &str) -> Result<()> {
    player.ensure_state(PlayerState::CharacterSelection)?;

    let index = player.characters.iter().position(|c| c.name == name);
    let index = index.ok_or(cxerr("Client sent invalid character name for selection"))?;
    player.select_character(index)
  }
}
