use jsonrpc_core::Error;
use rpc::JoinServiceStatus;

build_rpc_trait! {
  pub trait RpcServerApi {
    #[rpc(name = "status")]
    fn status(&self) -> Result<JoinServiceStatus, Error>;
  }
}
