use GameServerId;
use jsonrpc_core::Error;
use std::net::Ipv4Addr;

build_rpc_trait! {
  pub trait GameServerApi {
    #[rpc(name = "status")]
    fn status(&self) -> Result<GameServerStatus, Error>;

    #[rpc(name = "version")]
    fn version(&self) -> Result<&'static str, Error>;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameServerStatus {
  pub id: GameServerId,
  pub host: Ipv4Addr,
  pub port: u16,
  pub clients: usize,
  pub max_clients: usize,
  pub uptime: u64,
}

// impl GameServerStatus {
// pub fn load_factor(&self) -> f32 { (self.clients as f32) / (self.max_clients
// as f32) } }
