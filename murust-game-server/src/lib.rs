#![feature(proc_macro, generators)]

#[macro_use]
extern crate log;
extern crate tap;

extern crate failure;
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

// TODO: Determine how logging output should be
// TODO: Implement server state, allowing pause, stop resume etc
// TODO: Add macro for printing error 'Display' whilst logging error 'Debug'

pub use config::GameServerConfig;
pub use server::GameServer;

#[macro_use]
mod macros;
mod actions;
mod config;
mod context;
mod error;
mod handlers;
mod listener;
mod player;
pub mod rpc;
mod server;
mod util;
mod views;

/// The type of a server ID.
pub type GameServerId = u16;
