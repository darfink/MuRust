//! Protocol used between the Game Server and Client.

pub use self::client::Client;

pub mod client;
pub mod models;
pub mod server;
mod util;
mod visitors;

/// Serial identifier for the client.
pub type Serial = [u8; 16];

/// Version identifier for the protocol.
pub type Version = [u8; 5];

/// This protocol version.
pub static VERSION: Version = [0x30, 0x30, 0x30, 0x30, 0x30];
