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
  pub capacity: usize,
  pub clients: usize,
  pub host: Ipv4Addr,
  pub port: u16,
  pub uptime: u64,
}
