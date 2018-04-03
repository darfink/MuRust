use mugs;

// TODO: Ensure Game Servers are a specific version?
jsonrpc_client!(pub struct GameServerApi {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<mugs::rpc::GameServerStatus>;
});
