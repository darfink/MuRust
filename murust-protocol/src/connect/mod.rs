//! Protocol used between the Connect Server and Client.

pub use self::client::Client;

pub mod client;
pub mod models;
pub mod server;

/// Version identifier for the protocol.
pub type Version = [u8; 3];

/// This protocol version.
pub static VERSION: Version = [0x00, 0x00, 0x01];
