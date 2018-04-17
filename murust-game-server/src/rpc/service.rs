use error::Result;
use failure::ResultExt;
use jsonrpc_core::IoHandler;
use jsonrpc_http_server::{Server, ServerBuilder};
use rpc::api::GameServerApi;
use std::net::SocketAddr;

/// An RPC service instance.
pub struct RpcService {
  server: Server,
  uri: String,
}

impl RpcService {
  /// Spawns the RPC service on the HTTP protocol.
  pub fn spawn<T: GameServerApi>(socket: SocketAddr, api: T) -> Result<Self> {
    let mut io = IoHandler::new();
    io.extend_with(api.to_delegate());

    ServerBuilder::new(io)
      .start_http(&socket)
      .map(|server| {
        let uri = format!("http://{}", server.address());
        RpcService { server, uri }
      })
      .context("Failed to start RPC server")
      .map_err(Into::into)
  }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { &self.uri }

  /// Closes the service.
  pub fn close(self) { self.server.close(); }
}
