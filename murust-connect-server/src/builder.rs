use connect::ConnectService;
use rpc::{RpcHandler, RpcService};
use std::{io, net::{SocketAddr, SocketAddrV4}};
use {ClientManager, ConnectServer, GameServerBrowser};

/// A builder for the Connect Server.
pub struct ServerBuilder {
  socket_rpc: SocketAddr,
  socket_connect: SocketAddrV4,
}

impl ServerBuilder {
  /// Set's the Connect Service socket.
  pub fn socket_connect(mut self, socket: SocketAddrV4) -> Self {
    self.socket_connect = socket;
    self
  }

  /// Set's the RPC Service socket.
  pub fn socket_rpc(mut self, socket: SocketAddr) -> Self {
    self.socket_rpc = socket;
    self
  }

  /// Spawns the Connect & RPC services and returns a controller.
  pub fn spawn(self) -> io::Result<ConnectServer> {
    let server_browser = GameServerBrowser::new()?;
    let client_manager = ClientManager::new();

    client_manager.add_disconnect_listener(|socket| info!("Client<{}> disconnected", socket.ip()));
    client_manager.add_connect_listener(|socket| {
      info!("Client<{}> connected", socket.ip());
      true
    });

    let connect_service = ConnectService::spawn(
      self.socket_connect,
      server_browser.clone(),
      client_manager.clone(),
    );

    let rpc_service = RpcService::spawn(
      self.socket_rpc,
      RpcHandler::new(self.socket_connect, client_manager.clone()),
    )?;

    Ok(ConnectServer {
      server_browser,
      client_manager,
      connect_service,
      rpc_service,
    })
  }
}

impl Default for ServerBuilder {
  fn default() -> Self {
    ServerBuilder {
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_connect: "0.0.0.0:2004".parse().unwrap(),
    }
  }
}
