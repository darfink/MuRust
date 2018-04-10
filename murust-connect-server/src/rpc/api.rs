use jsonrpc_core::Error;
use std::net::Ipv4Addr;

build_rpc_trait! {
  pub trait ConnectServerApi {
    #[rpc(name = "status")]
    fn status(&self) -> Result<ConnectServerStatus, Error>;

    #[rpc(name = "version")]
    fn version(&self) -> Result<&'static str, Error>;
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectServerStatus {
  pub clients: usize,
  pub host: Ipv4Addr,
  pub port: u16,
  pub uptime: u64,
}
