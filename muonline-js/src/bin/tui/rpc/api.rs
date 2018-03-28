use mujs::rpc::JoinServerStatus;

jsonrpc_client!(pub struct JoinServerApi {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<JoinServerStatus>;
});
