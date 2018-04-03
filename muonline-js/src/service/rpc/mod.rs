use self::api::{JoinServerApi, JoinServerStatus};
use controller::JoinServerController;
use jsonrpc_core::{Error, IoHandler};
use jsonrpc_http_server;
use jsonrpc_http_server::ServerBuilder;
use std::io;
use std::net::SocketAddr;

pub mod api;

/// An RPC service instance.
pub struct RpcService {
  server: jsonrpc_http_server::Server,
  uri: String,
}

impl RpcService {
  /// Spawns the RPC service on the HTTP protocol.
  pub fn spawn(socket: SocketAddr, controller: JoinServerController) -> io::Result<Self> {
    let mut io = IoHandler::new();
    io.extend_with(controller.to_delegate());

    ServerBuilder::new(io).start_http(&socket).map(|server| {
      let uri = format!("http://{}", server.address());
      RpcService { server, uri }
    })
  }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { &self.uri }

  /// Closes the service.
  pub fn close(self) { self.server.close(); }
}

impl JoinServerApi for JoinServerController {
  fn status(&self) -> Result<JoinServerStatus, Error> {
    let context = self.context();
    let socket = self.socket();

    Ok(JoinServerStatus {
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.uptime().as_secs(),
      clients: context.number_of_clients(),
    })
  }

  fn version(&self) -> Result<&'static str, Error> { Ok(env!("CARGO_PKG_VERSION")) }
}
