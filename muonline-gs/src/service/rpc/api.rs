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

#[derive(Serialize, Deserialize, Debug)]
pub struct GameServerStatus {
  pub id: u16,
  pub host: Ipv4Addr,
  pub port: u16,
  pub clients: usize,
  pub max_clients: usize,
  pub uptime: u64,
}
