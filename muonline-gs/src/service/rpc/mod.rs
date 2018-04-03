use self::api::{GameServerApi, GameServerStatus};
use controller::GameServerController;
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
  pub fn spawn(socket: SocketAddr, controller: GameServerController) -> io::Result<Self> {
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

impl GameServerApi for GameServerController {
  fn status(&self) -> Result<GameServerStatus, Error> {
    let context = self.context();
    let socket = self.socket();

    Ok(GameServerStatus {
      id: self.id(),
      host: *socket.ip(),
      port: socket.port(),
      clients: context.number_of_clients(),
      max_clients: self.max_clients(),
      uptime: self.uptime().as_secs(),
    })
  }

  fn version(&self) -> Result<&'static str, Error> {
    Ok(env!("CARGO_PKG_VERSION"))
  }
}
