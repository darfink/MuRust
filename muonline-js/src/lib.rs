#[macro_use]
extern crate log;
#[macro_use]
extern crate closet;

extern crate futures;
extern crate tokio;

extern crate muonline_packet as mupack;
extern crate muonline_packet_codec as mucodec;
extern crate muonline_protocol as protocol;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

pub use self::builder::ServerBuilder;
use std::io;

mod builder;
pub mod rpc;
mod service;

/// An implementation of a Join Server.
pub struct JoinServer {
  service: service::JoinService,
  rpc: jsonrpc_http_server::Server,
  rpc_uri: String,
}

impl JoinServer {
  /// Spawns a new Join Server using defaults.
  pub fn spawn() -> io::Result<Self> {
    Self::builder().spawn()
  }

  /// Returns a builder for the Join Server.
  pub fn builder() -> ServerBuilder {
    ServerBuilder::new()
  }

  /// Returns the URI for the RPC server.
  pub fn uri(&self) -> &str {
    &self.rpc_uri
  }

  /// Waits for the server to finish.
  pub fn wait(self) -> io::Result<()> {
    self.service.wait()?;
    self.rpc.close();
    Ok(())
  }

  /// Closes the server.
  pub fn close(self) -> io::Result<()> {
    self.service.close()?;
    self.rpc.close();
    Ok(())
  }
}
