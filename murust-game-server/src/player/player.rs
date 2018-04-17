use context::GameServerContext;
use error::{cxerr, Result};
use handlers::PacketHandlerCore;
use murust_data_model::entities::{Account, Character};
use murust_data_model::types::ObjectId;
use player::PlayerState;
use std::sync::Arc;
use views::PlayerView;

pub struct Player {
  // TODO: Set ID externally after join packet?
  pub id: ObjectId,
  pub account: Option<Account>,
  pub characters: Vec<Character>,
  pub character_index: Option<usize>,
  // TODO: Abstract away the 'server' part of this?
  pub context: GameServerContext,
  pub state: PlayerState,
  pub player_view: PlayerView,
  pub packet_handler: Arc<PacketHandlerCore>,
}

// TODO: On player state → CharacterSelection, send MOTD.
impl Player {
  pub fn new(id: ObjectId, context: GameServerContext, player_view: PlayerView) -> Player {
    let packet_handler = context.packet_handler();
    Player {
      id,
      account: None,
      characters: Vec::new(),
      character_index: None,
      context,
      state: PlayerState::LoginScreen,
      packet_handler,
      player_view,
    }
  }

  /// Returns the player's account.
  pub fn account(&self) -> Result<&Account> {
    self
      .account
      .as_ref()
      .ok_or(cxerr("Invalid access to account when not available"))
  }

  /// Returns the player's selected character.
  pub fn character(&self) -> Result<&Character> {
    self
      .character_index
      .and_then(|i| self.characters.get(i))
      .ok_or(cxerr("Invalid access to character when none selected"))
  }

  pub fn select_character(&mut self, character_index: usize) -> Result<()> {
    self.character_index = Some(character_index);
    self.player_entered_world()
  }

  pub fn ensure_state(&self, state: PlayerState) -> Result<()> {
    if self.state != state {
      Err(cxerr(format!(
        "Invalid player state {}, expected {}",
        self.state, state
      )))
    } else {
      Ok(())
    }
  }

  fn player_entered_world(&mut self) -> Result<()> {
    self.player_view.update_character_info(self)?;
    self.player_view.update_inventory_list(self)?;
    Ok(())
  }
}
