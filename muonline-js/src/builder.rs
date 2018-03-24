use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use rpc::RpcServer;
use service::{JoinService, JoinServiceContext};
use JoinServer;

pub struct ServerBuilder {
  socket_rpc: SocketAddr,
  socket_service: SocketAddrV4,
  gs_remote: Vec<SocketAddrV4>,
  gs_local: Vec<u16>,
}

impl ServerBuilder {
  pub fn new() -> Self {
    ServerBuilder {
      socket_rpc: "127.0.0.1:0".parse().unwrap(),
      socket_service: "0.0.0.0:2004".parse().unwrap(),
      gs_remote: Vec::new(),
      gs_local: Vec::new(),
    }
  }

  pub fn service(mut self, socket: SocketAddrV4) -> Self {
    self.socket_service = socket;
    self
  }

  pub fn rpc(mut self, socket: SocketAddr) -> Self {
    self.socket_rpc = socket;
    self
  }

  pub fn remote(mut self, socket: SocketAddrV4) -> Self {
    self.gs_remote.push(socket);
    self
  }

  pub fn local(mut self, code: u16) -> Self {
    self.gs_local.push(code);
    self
  }

  pub fn spawn(self) -> io::Result<JoinServer> {
    let context = Arc::new(JoinServiceContext::new(self.socket_service));
    let (rpc, rpc_uri) = RpcServer::new(context.clone()).spawn(self.socket_rpc)?;
    let service = JoinService::spawn(context.clone());

    // TODO: Where should this be at?
    info!("RPC servicing at {}", rpc_uri);

    Ok(JoinServer {
      rpc,
      rpc_uri,
      service,
    })
  }
}
