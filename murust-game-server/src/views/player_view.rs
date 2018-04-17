use error::Result;
use failure::ResultExt;
use futures::sync::mpsc;
use muonline_packet::{Packet, PacketEncodable};
use murust_data_model::entities::Character;
use murust_data_model::types::Class;
use player::Player;

#[derive(Debug, Copy, Clone)]
pub enum LoginResult {
  Success,
  AlreadyConnected,
  IncorrectPassword,
  InvalidAccount,
  TooManyAttempts,
}

#[derive(Debug, Copy, Clone)]
pub enum CharacterCreateResult<'a> {
  Success(&'a Character),
  LimitReached,
  InvalidName,
}

#[derive(Debug, Copy, Clone)]
pub enum CharacterDeleteResult {
  Success,
  GuildCharacter,
  InvalidSecurityCode,
  Blocked,
}

pub struct PlayerView {
  // TODO: Abstract this to a stream.
  output: mpsc::UnboundedSender<Packet>,
}

impl PlayerView {
  pub fn new(output: mpsc::UnboundedSender<Packet>) -> Self { PlayerView { output } }

  pub fn show_login_result(&self, result: LoginResult) -> Result<()> {
    use protocol::game::server::AccountLoginResult;
    let packet = match result {
      LoginResult::Success => AccountLoginResult::Success,
      LoginResult::AlreadyConnected => AccountLoginResult::AlreadyConnected,
      LoginResult::IncorrectPassword => AccountLoginResult::IncorrectPassword,
      LoginResult::InvalidAccount => AccountLoginResult::InvalidAccount,
      LoginResult::TooManyAttempts => AccountLoginResult::TooManyAttempts,
    };
    self.send_packet(packet)
  }

  pub fn show_character_list(&self, player: &Player) -> Result<()> {
    use protocol::game::server::CharacterList;
    self.send_packet(CharacterList::new(Class::FairyElf, &player.characters))
  }

  pub fn show_character_create_response(&self, result: CharacterCreateResult) -> Result<()> {
    use protocol::game::server;
    let packet = match result {
      CharacterCreateResult::Success(character) => {
        server::CharacterCreateResult::success(character)
      },
      CharacterCreateResult::LimitReached => server::CharacterCreateResult::LimitReached,
      CharacterCreateResult::InvalidName => server::CharacterCreateResult::InvalidName,
    };
    self.send_packet(packet)
  }

  pub fn show_character_delete_response(&self, result: CharacterDeleteResult) -> Result<()> {
    use protocol::game::server;
    let packet = match result {
      CharacterDeleteResult::Success => server::CharacterDeleteResult::Success,
      CharacterDeleteResult::GuildCharacter => server::CharacterDeleteResult::GuildCharacter,
      CharacterDeleteResult::InvalidSecurityCode => {
        server::CharacterDeleteResult::InvalidSecurityCode
      },
      CharacterDeleteResult::Blocked => server::CharacterDeleteResult::Blocked,
    };
    self.send_packet(packet)
  }

  pub fn update_character_info(&self, player: &Player) -> Result<()> {
    use protocol::game::server::CharacterInfo;
    self.send_packet(CharacterInfo::new(player.character()?))
  }

  // TODO: Move this somewhere else?
  pub fn update_inventory_list(&self, player: &Player) -> Result<()> {
    use protocol::game::server::InventoryList;
    self.send_packet(InventoryList::new(player.character()?))
  }

  fn send_packet<P: PacketEncodable>(&self, packet: P) -> Result<()> {
    let packet = packet
      .to_packet()
      .context("Failed to serialize packet to client")?;
    self
      .output
      .unbounded_send(packet)
      .context("Failed to send client packet using server channel")
      .map_err(Into::into)
  }
}
