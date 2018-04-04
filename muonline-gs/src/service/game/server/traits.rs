use futures::{Future, IntoFuture, Sink};
use mupack::{self, PacketEncodable};
use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddrV4, SocketAddr};
use std::io;

pub trait PacketSink: Sink<SinkItem = mupack::Packet, SinkError = io::Error> {
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = io::Error> + Send>;
}

impl<S> PacketSink for S
where
  S: Sink<SinkItem = mupack::Packet, SinkError = io::Error> + Send + 'static,
{
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = io::Error> + Send> {
    Box::new(
      packet
        .to_packet()
        .into_future()
        .and_then(move |packet| self.send(packet)),
    )
  }
}

pub trait SocketProvider {
  fn ipv4socket(&self) -> io::Result<SocketAddrV4>;
}

impl SocketProvider for TcpListener {
  fn ipv4socket(&self) -> io::Result<SocketAddrV4> {
    match self.local_addr()? {
      SocketAddr::V4(socket) => Ok(socket),
      _ => Err(io::ErrorKind::InvalidInput.into()),
    }
  }
}

impl SocketProvider for TcpStream {
  fn ipv4socket(&self) -> io::Result<SocketAddrV4> {
    match self.peer_addr()? {
      SocketAddr::V4(socket) => Ok(socket),
      _ => Err(io::ErrorKind::InvalidInput.into()),
    }
  }
}
