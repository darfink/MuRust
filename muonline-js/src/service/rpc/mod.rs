use self::api::{JoinServerApi, JoinServerStatus};
use jsonrpc_core::{Error, IoHandler};
use jsonrpc_http_server;
use jsonrpc_http_server::ServerBuilder;
use service::JoinServiceInterface;
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
  pub fn spawn<T: JoinServiceInterface>(socket: SocketAddr, jsi: T) -> io::Result<Self> {
    let mut io = IoHandler::new();
    io.extend_with(jsi.to_delegate());

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

impl<T: JoinServiceInterface> JoinServerApi for T {
  fn status(&self) -> Result<JoinServerStatus, Error> {
    let socket = self.socket();

    Ok(JoinServerStatus {
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.uptime().as_secs(),
      clients: self.number_of_clients(),
    })
  }
}
