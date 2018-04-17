use error::Result;
use player::{Player, PlayerState};

pub struct CharacterListAction;

impl CharacterListAction {
  pub fn list(&self, player: &mut Player) -> Result<()> {
    if player.state.try_advance_to(PlayerState::CharacterSelection) {
      player.player_view.show_character_list(player)?;
    }
    Ok(())
  }
}
