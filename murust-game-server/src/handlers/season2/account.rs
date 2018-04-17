use super::PacketHandler;
use actions::LoginAction;
use error::Result;
use murust_service::ServiceManager;
use player::Player;
use protocol::game::Client;

pub struct AccountHandler {
  login_action: LoginAction,
  // logout_action: LogoutAction,
}

impl AccountHandler {
  pub fn new(service_manager: &ServiceManager) -> Self {
    AccountHandler {
      login_action: LoginAction::new(
        service_manager.account_service(),
        service_manager.character_service(),
      ),
      // logout_action: LogoutAction::new(service_manager.account_service()),
    }
  }
}

impl PacketHandler for AccountHandler {
  fn handle_packet(&self, player: &mut Player, packet: &Client) -> Result<bool> {
    match packet {
      // TODO: Determine packet handler version
      Client::AccountLoginRequest(request) => {
        self
          .login_action
          .login(player, &request.username, &request.password)?
      },
      // Client::AccountLogoutRequest(request) =>
      //  self.logout_action.logout(player)?,
      _ => return Ok(false),
    }
    Ok(true)
  }
}
