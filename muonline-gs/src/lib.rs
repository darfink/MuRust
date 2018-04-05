#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
extern crate tap;

extern crate futures;
extern crate tokio;

extern crate muonline_db as mudb;
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

#[macro_use]
mod macros;
mod builder;
mod controller;
mod service;
pub mod rpc {
  // Re-export the RPC API
  pub use service::rpc::api::*;
}

/// An implementation of a Game Server.
pub struct GameServer {
  game_service: service::GameService,
  rpc_service: service::RpcService,
}

impl GameServer {
  /// Returns a builder for the Game Server.
  pub fn builder(id: u16) -> ServerBuilder { ServerBuilder::new(id) }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { self.rpc_service.uri() }

  /// Closes the server.
  pub fn close(self) -> io::Result<()> {
    let result = self.game_service.close();
    self.rpc_service.close();
    result
  }

  /// Will block, waiting for the server to finish.
  pub fn wait(self) -> io::Result<()> {
    let result = self.game_service.wait();
    self.rpc_service.close();
    result
  }
}
