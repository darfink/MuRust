use error::Result;
use failure::ResultExt;
use murust_data_model::types::Class;
use murust_service::{CharacterCreateError, CharacterService};
use player::{Player, PlayerState};
use views::CharacterCreateResult;

pub struct CharacterCreateAction {
  character_service: CharacterService,
}

impl CharacterCreateAction {
  pub fn new(character_service: CharacterService) -> Self {
    CharacterCreateAction { character_service }
  }

  pub fn create(&self, player: &mut Player, name: &str, class: Class) -> Result<()> {
    player.ensure_state(PlayerState::CharacterSelection)?;

    let create_request = self
      .character_service
      .create(&name, class, player.account()?.id)
      .context("Character service failed to create new character")?;

    match create_request {
      Ok(character) => {
        // The borrow checker is so stupid sometimes
        {
          let response = CharacterCreateResult::Success(&character);
          player.player_view.show_character_create_response(response)?;
        }
        player.characters.push(character);
        Ok(())
      },
      Err(error) => {
        let response = self.map_error_to_result(error);
        player.player_view.show_character_create_response(response)
      },
    }
  }

  /// Converts a character service creation error to a result.
  fn map_error_to_result(&self, error: CharacterCreateError) -> CharacterCreateResult {
    match error {
      CharacterCreateError::LimitReached => CharacterCreateResult::LimitReached,
      CharacterCreateError::InvalidName | CharacterCreateError::OccupiedName => {
        CharacterCreateResult::InvalidName
      },
    }
  }
}
