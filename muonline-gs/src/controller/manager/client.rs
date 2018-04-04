use std::net::SocketAddrV4;

/// A representation of a service client.
#[derive(Debug)]
pub struct Client {
  pub id: usize,
  pub socket: SocketAddrV4,
}

impl Client {
  /// Constructs a new client instance.
  pub fn new(id: usize, socket: SocketAddrV4) -> Self { Client { id, socket } }
}
