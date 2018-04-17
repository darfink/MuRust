use error::Result;
use failure::ResultExt;
use murust_service::{AccountLoginError, AccountService, CharacterService};
use player::{Player, PlayerState};
use views::LoginResult;

pub struct LoginAction {
  characters: CharacterService,
  accounts: AccountService,
}

impl LoginAction {
  // TODO: Is character service dependency desired here?
  pub fn new(accounts: AccountService, characters: CharacterService) -> Self {
    LoginAction {
      accounts,
      characters,
    }
  }

  pub fn login(&self, player: &mut Player, username: &str, password: &str) -> Result<()> {
    player.ensure_state(PlayerState::LoginScreen)?;

    // TODO: Check if user is banned/server is preparing? Admin...
    let account_request = self
      .accounts
      .login(username, password)
      .context("Account service failed to process login")?;

    let result = match account_request {
      Err(error) => self.map_error_to_result(error),
      Ok(account) => {
        let characters = self
          .characters
          .find_by_account_id(account.id)
          .context("Character service failed to provide list")?;

        player.account = Some(account);
        player.characters = characters;
        LoginResult::Success
      },
    };

    player.player_view.show_login_result(result)
  }

  /// Converts a login service error to a result.
  fn map_error_to_result(&self, error: AccountLoginError) -> LoginResult {
    match error {
      // TODO: It should not be revealed that the password was incorrect
      AccountLoginError::InvalidUsername => LoginResult::InvalidAccount,
      AccountLoginError::InvalidPassword(_) => LoginResult::IncorrectPassword,
      AccountLoginError::AlreadyConnected(_) => LoginResult::AlreadyConnected,
      AccountLoginError::Throttled(_) => LoginResult::TooManyAttempts,
    }
  }
}
