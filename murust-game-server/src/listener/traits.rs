use failure::{Context, Error, Fail, ResultExt};
use futures::{Future, IntoFuture, Sink, Stream};
use muonline_packet::{Packet, PacketEncodable};
use std::{io, net::{SocketAddr, SocketAddrV4}};
use tokio::net::{TcpListener, TcpStream};

pub trait PacketSink: Sink<SinkItem = Packet, SinkError = io::Error> {
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = Error> + Send>;
}

impl<S> PacketSink for S
where
  S: Sink<SinkItem = Packet, SinkError = io::Error> + Send + 'static,
{
  fn send_packet<P: PacketEncodable>(
    self,
    packet: &P,
  ) -> Box<Future<Item = Self, Error = Error> + Send> {
    Box::new(
      packet
        .to_packet()
        .into_future()
        .map_err(|error| error.context("Failed to serialize packet").into())
        .and_then(move |packet| {
          self
            .send(packet)
            .map_err(|error| error.context("Failed to send packet").into())
        }),
    )
  }
}

pub trait PacketStream: Stream<Item = Packet, Error = Error> {
  fn next_packet(self) -> Box<Future<Item = (Self::Item, Self), Error = Self::Error> + Send>;
}

impl<S> PacketStream for S
where
  S: Stream<Item = Packet, Error = Error> + Send + 'static,
{
  fn next_packet(self) -> Box<Future<Item = (Self::Item, Self), Error = Self::Error> + Send> {
    Box::new(
      self
        .into_future()
        .map_err(|(err, _)| err)
        .and_then(move |(item, stream)| {
          item
            .map(move |item| (item, stream))
            .ok_or_else(|| Context::new("Incoming packet stream ended prematurely").into())
            .into_future()
        }),
    )
  }
}

pub trait SocketProvider {
  fn ipv4socket(&self) -> Result<SocketAddrV4, Error>;
}

impl SocketProvider for TcpListener {
  fn ipv4socket(&self) -> Result<SocketAddrV4, Error> {
    self
      .local_addr()
      .context("Could not evaluate server address")
      .and_then(|socket| match socket {
        SocketAddr::V4(socket) => Ok(socket),
        _ => Err(Context::new("Invalid server socket type")),
      })
      .map_err(Into::into)
  }
}

impl SocketProvider for TcpStream {
  fn ipv4socket(&self) -> Result<SocketAddrV4, Error> {
    self
      .peer_addr()
      .context("Could not evaluate client address")
      .and_then(|socket| match socket {
        SocketAddr::V4(socket) => Ok(socket),
        _ => Err(Context::new("Invalid client socket type")),
      })
      .map_err(Into::into)
  }
}
