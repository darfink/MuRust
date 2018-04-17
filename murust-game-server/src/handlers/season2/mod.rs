use error::Result;
use failure::ResultExt;
use handlers::PacketHandlerCore;
use muonline_packet::Packet;
use murust_service::ServiceManager;
use player::Player;
use protocol::game::Client;
use protocol::game::VERSION;

mod account;
mod lobby;

trait PacketHandler {
  /// Analyzes an incoming packet and returns whether it was handled or not.
  fn handle_packet(&self, player: &mut Player, packet: &Client) -> Result<bool>;
}

pub struct Season2PacketHandler {
  handlers: Vec<Box<PacketHandler + Send + Sync>>,
}

impl Season2PacketHandler {
  /// Constructs a new season 2 packet handler.
  pub fn new(service_manager: &ServiceManager) -> Self {
    Season2PacketHandler {
      handlers: vec![
        Box::new(account::AccountHandler::new(service_manager)),
        Box::new(lobby::CharacterLobbyHandler::new(service_manager)),
      ],
    }
  }
}

impl PacketHandlerCore for Season2PacketHandler {
  /// The protocol version used by season 2.
  fn version(&self) -> [u8; 5] { VERSION }

  /// Dispatches an incoming packet to an appropriate handler.
  fn handle_packet(&self, player: &mut Player, packet: Packet) -> Result<()> {
    let client = Client::from_packet(&packet).context("Client sent a corrupted network packet")?;

    for handler in &self.handlers {
      if handler.handle_packet(player, &client)? {
        break;
      }
    }

    // TODO: Add warning for unhandled packets
    if client.is_unknown() {
      info!("Unknown packet: {:#?}", packet);
    }

    Ok(())
  }
}
