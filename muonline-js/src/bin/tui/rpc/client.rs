use mujs::rpc::JoinServiceStatus;

jsonrpc_client!(pub struct JoinServiceClient {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<JoinServiceStatus>;
});
