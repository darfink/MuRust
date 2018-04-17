use GameServerId;
use std::net::SocketAddrV4;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameServerConfig {
  pub id: GameServerId,
  pub socket: SocketAddrV4,
  pub maximum_players: usize,
}
