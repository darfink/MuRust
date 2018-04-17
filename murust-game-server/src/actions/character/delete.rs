use error::{cxerr, Result};
use failure::ResultExt;
use murust_service::{CharacterDeleteError, CharacterService};
use player::{Player, PlayerState};
use util::RemoveItemBy;
use views::CharacterDeleteResult;

pub struct CharacterDeleteAction {
  character_service: CharacterService,
}

impl CharacterDeleteAction {
  pub fn new(character_service: CharacterService) -> Self {
    CharacterDeleteAction { character_service }
  }

  pub fn delete(&self, player: &mut Player, name: &str, security_code: &str) -> Result<()> {
    player.ensure_state(PlayerState::CharacterSelection)?;
    let response = self.delete_impl(player, name, security_code)?;
    player.player_view.show_character_delete_response(response)
  }

  pub fn delete_impl(
    &self,
    player: &mut Player,
    name: &str,
    security_code: &str,
  ) -> Result<CharacterDeleteResult> {
    // TODO: Avoid allocation here, change deserialization?
    // TODO: These attempts should perhaps be throttled as well?
    if player.account()?.security_code.to_string() != security_code {
      info!("Client entered an invalid security code for character deletion");
      return Ok(CharacterDeleteResult::InvalidSecurityCode);
    }

    let character = player
      .characters
      .remove_item_by(|c| c.name == name)
      .ok_or(cxerr("Client sent invalid character name for deletion"))?;

    let delete_request = self
      .character_service
      .delete(character)
      .context("Character service failed to delete character")?;

    Ok(match delete_request {
      Ok(_) => CharacterDeleteResult::Success,
      Err((_, error)) => self.map_error_to_result(error),
    })
  }

  /// Converts a character service deletion error to a result.
  fn map_error_to_result(&self, error: CharacterDeleteError) -> CharacterDeleteResult {
    match error {
      CharacterDeleteError::GuildCharacter => CharacterDeleteResult::GuildCharacter,
      CharacterDeleteError::Blocked => CharacterDeleteResult::Blocked,
    }
  }
}
