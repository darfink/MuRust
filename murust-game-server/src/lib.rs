#![feature(proc_macro, generators)]

#[macro_use]
extern crate log;
extern crate tap;

extern crate futures_await as futures;
extern crate muonline_packet;
extern crate muonline_packet_codec;
extern crate murust_data_model;
extern crate murust_protocol as protocol;
extern crate murust_service;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

// TODO: Terminology service/server + ServiceManager
// TODO: Replace all unwraps with expect
// TODO: Figure out Error string formatting
// TODO: Figure out what Error type to use/propagate
// TODO: Determine how logging output should be

pub use builder::ServerBuilder;
use clients::ClientManager;
use info::ServerInfo;
use murust_service::ServiceManager;
use std::io;

#[macro_use]
mod macros;
mod builder;
mod clients;
mod game;
mod info;
mod rpc;

/// The type of a server ID.
pub type GameServerId = u16;

/// An implementation of a Game Server.
pub struct GameServer {
  client_manager: ClientManager,
  game_service: game::GameService,
  rpc_service: rpc::RpcService,
}

impl GameServer {
  /// Returns a builder for the Game Server.
  pub fn builder(server_id: GameServerId, service_manager: ServiceManager) -> ServerBuilder {
    ServerBuilder::new(server_id, service_manager)
  }

  /// Returns the current number of connected clients.
  pub fn clients(&self) -> usize { self.client_manager.len() }

  /// Returns the URI of the RPC service.
  pub fn uri(&self) -> &str { self.rpc_service.uri() }

  /// Stops the server.
  pub fn stop(self) -> io::Result<()> {
    let result = self.game_service.stop();
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
