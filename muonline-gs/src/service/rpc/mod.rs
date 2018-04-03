use self::api::{GameServerApi, GameServerStatus};
use jsonrpc_core::{Error, IoHandler};
use jsonrpc_http_server;
use jsonrpc_http_server::ServerBuilder;
use service::GameServiceInterface;
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
  pub fn spawn<T: GameServiceInterface>(socket: SocketAddr, jsi: T) -> io::Result<Self> {
    let mut io = IoHandler::new();
    io.extend_with(jsi.to_delegate());

    ServerBuilder::new(io).start_http(&socket).map(|server| {
      let uri = format!("http://{}", server.address());
      info!("RPC servicing at {}", &uri);
      RpcService { server, uri }
    })
  }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { &self.uri }

  /// Closes the service.
  pub fn close(self) { self.server.close(); }
}

impl<T: GameServiceInterface> GameServerApi for T {
  fn status(&self) -> Result<GameServerStatus, Error> {
    let socket = self.socket();

    Ok(GameServerStatus {
      id: self.id(),
      capacity: self.capacity(),
      host: *socket.ip(),
      port: socket.port(),
      uptime: self.uptime().as_secs(),
      clients: self.number_of_clients(),
    })
  }

  fn version(&self) -> Result<&'static str, Error> {
    Ok(env!("CARGO_PKG_VERSION"))
  }
}
