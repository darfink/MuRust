use jsonrpc_core::Error;
use std::net::Ipv4Addr;

build_rpc_trait! {
  pub trait JoinServerApi {
    #[rpc(name = "status")]
    fn status(&self) -> Result<JoinServerStatus, Error>;

    #[rpc(name = "version")]
    fn version(&self) -> Result<&'static str, Error>;
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinServerStatus {
  pub clients: usize,
  pub host: Ipv4Addr,
  pub port: u16,
  pub uptime: u64,
}
