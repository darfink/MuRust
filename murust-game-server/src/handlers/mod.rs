use error::Result;
use muonline_packet::Packet;
use murust_service::ServiceManager;
use player::Player;

mod season2;

pub trait PacketHandlerCore: Send + Sync {
  /// The protocl version the core uses.
  fn version(&self) -> [u8; 5];

  /// Processes an handles an incoming packet.
  fn handle_packet(&self, player: &mut Player, packet: Packet) -> Result<()>;
}

pub fn default(service_manager: &ServiceManager) -> impl PacketHandlerCore {
  // TODO: Create an aggregate of season packet handlers.
  season2::Season2PacketHandler::new(service_manager)
}
