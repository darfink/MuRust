#![feature(proc_macro, generators)]

#[macro_use]
extern crate log;
extern crate tap;

extern crate futures_await as futures;
extern crate muonline_packet;
extern crate muonline_packet_codec;
extern crate murust_protocol as protocol;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

// TODO: Replace all unwraps with expect
// TODO: Figure out Error string formatting
// TODO: Figure out what Error type to use/propagate
// TODO: Determine how logging output should be

use browser::GameServerBrowser;
pub use builder::ServerBuilder;
use clients::ClientManager;
use std::io;

#[macro_use]
mod macros;
mod browser;
mod builder;
mod clients;
mod connect;
mod rpc;

/// An implementation of a Connect Server.
pub struct ConnectServer {
  server_browser: GameServerBrowser,
  client_manager: ClientManager,
  connect_service: connect::ConnectService,
  rpc_service: rpc::RpcService,
}

impl ConnectServer {
  /// Spawns a new Connect Server using defaults.
  pub fn spawn() -> io::Result<Self> { Self::builder().spawn() }

  /// Returns a builder for the Connect Server.
  pub fn builder() -> ServerBuilder { ServerBuilder::default() }

  /// Adds a game server to the join server.
  pub fn add_game_server(&mut self, uri: &str) -> io::Result<()> { self.server_browser.add(uri) }

  /// Returns the current number of connected clients.
  pub fn clients(&self) -> usize { self.client_manager.len() }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { self.rpc_service.uri() }

  /// Stops the server.
  pub fn stop(self) -> io::Result<()> {
    let result = self.connect_service.stop();
    self.rpc_service.close();
    result
  }

  /// Will block, waiting for the server to finish.
  pub fn wait(self) -> io::Result<()> {
    let result = self.connect_service.wait();
    self.rpc_service.close();
    result
  }
}
