use futures::{Future, IntoFuture, Sink, Stream};
use muonline_packet::{Packet, PacketEncodable};
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use tokio::net::{TcpListener, TcpStream};

pub trait PacketSink: Sink<SinkItem = Packet, SinkError = io::Error> {
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = Self::SinkError> + Send>;
}

impl<S> PacketSink for S
where
  S: Sink<SinkItem = Packet, SinkError = io::Error> + Send + 'static,
{
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = Self::SinkError> + Send> {
    Box::new(
      packet
        .to_packet()
        .into_future()
        .and_then(move |packet| self.send(packet)),
    )
  }
}

pub trait PacketStream: Stream<Item = Packet, Error = io::Error> {
  fn next_packet(self) -> Box<Future<Item = (Self::Item, Self), Error = Self::Error> + Send>;
}

impl<S> PacketStream for S
where
  S: Stream<Item = Packet, Error = io::Error> + Send + 'static,
{
  fn next_packet(self) -> Box<Future<Item = (Self::Item, Self), Error = Self::Error> + Send> {
    Box::new(
      self
        .into_future()
        .map_err(|(err, _)| err)
        .and_then(move |(item, stream)| {
          item
            .map(move |item| (item, stream))
            .ok_or(io::ErrorKind::ConnectionReset.into())
            .into_future()
        }),
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
