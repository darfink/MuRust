use super::PacketHandler;
use actions::{CharacterCreateAction, CharacterDeleteAction, CharacterListAction,
              CharacterSelectAction};
use error::Result;
use murust_service::ServiceManager;
use player::Player;
use protocol::game::Client;

pub struct CharacterLobbyHandler {
  list_action: CharacterListAction,
  create_action: CharacterCreateAction,
  delete_action: CharacterDeleteAction,
  select_action: CharacterSelectAction,
}

impl CharacterLobbyHandler {
  pub fn new(service_manager: &ServiceManager) -> Self {
    CharacterLobbyHandler {
      list_action: CharacterListAction,
      create_action: CharacterCreateAction::new(service_manager.character_service()),
      delete_action: CharacterDeleteAction::new(service_manager.character_service()),
      select_action: CharacterSelectAction,
    }
  }
}

impl PacketHandler for CharacterLobbyHandler {
  fn handle_packet(&self, player: &mut Player, packet: &Client) -> Result<bool> {
    match packet {
      Client::CharacterListRequest => self.list_action.list(player)?,
      Client::CharacterCreate(request) => {
        self
          .create_action
          .create(player, &request.name, request.class)?
      },
      Client::CharacterDelete(request) => {
        self
          .delete_action
          .delete(player, &request.name, &request.security_code)?
      },
      Client::CharacterSelect(request) => self.select_action.select(player, &request.name)?,
      _ => return Ok(false),
    }
    Ok(true)
  }
}
