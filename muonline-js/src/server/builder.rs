use std::net::SocketAddrV4;
use std::sync::Arc;
use futures::sync::oneshot;
use ipc::{Ipc, DefaultIpc};
use super::{JoinServer, GameServer};

pub struct Builder {
  socket: SocketAddrV4,
  remote: Vec<SocketAddrV4>,
  local: Vec<u16>,
  ipc: Arc<Ipc>,
}

impl Builder {
  pub fn new(socket: SocketAddrV4) -> Self {
    Builder {
      socket,
      remote: Vec::new(),
      local: Vec::new(),
      ipc: Arc::new(DefaultIpc::new()),
    }
  }

  pub fn remote(mut self, socket: SocketAddrV4) -> Self {
    self.remote.push(socket);
    self
  }

  pub fn local(mut self, code: u16) -> Self {
    self.local.push(code);
    self
  }

  pub fn ipc(mut self, ipc: Arc<Ipc>) -> Self {
    self.ipc = ipc;
    self
  }

  pub fn build(self) -> (JoinServer, oneshot::Sender<()>) {
    let (tx, rx) = oneshot::channel();
    let server = JoinServer {
      socket: self.socket,
      servers: self.local.iter()
        .map(|code| GameServer { code: *code })
        .collect::<Vec<_>>(),
      cancel: rx,
      ipc: self.ipc,
    };

    (server, tx)
  }
}