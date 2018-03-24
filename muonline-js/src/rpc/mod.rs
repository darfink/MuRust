use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use jsonrpc_http_server;
use jsonrpc_core::{Error, IoHandler};
use jsonrpc_http_server::ServerBuilder;
use service::JoinServiceContext;
use self::api::RpcServerApi;
pub use self::models::JoinServiceStatus;

mod api;
mod models;

/// An RPC server instance.
pub(crate) struct RpcServer {
  // TODO: Make JoinServiceContext a trait
  context: Arc<JoinServiceContext>,
}

impl RpcServer {
  /// Constructs a new RPC server instance.
  pub fn new(context: Arc<JoinServiceContext>) -> Self {
    RpcServer { context }
  }

  /// Starts the RPC server instance on the HTTP protocol.
  pub fn spawn(self, socket: SocketAddr) -> io::Result<(jsonrpc_http_server::Server, String)> {
    let mut io = IoHandler::new();
    io.extend_with(self.to_delegate());

    let server = ServerBuilder::new(io).start_http(&socket)?;
    let uri = format!("http://{}", server.address());
    Ok((server, uri))
  }
}

impl RpcServerApi for RpcServer {
  fn status(&self) -> Result<JoinServiceStatus, Error> {
    let socket = self.context.socket();

    Ok(JoinServiceStatus {
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.context.uptime().as_secs(),
      clients: self.context.number_of_clients(),
    })
  }
}
