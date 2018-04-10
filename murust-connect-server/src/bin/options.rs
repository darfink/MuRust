use std::net::{IpAddr, Ipv4Addr};

// TODO: Parse remote game server URLs.
#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "mucs", about = "Mu Online Season 2 Connect Server")]
pub struct Options {
  #[structopt(long = "rpc-host", value_name = "host", help = "Bind RPC to this IP address",
              default_value = "127.0.0.1")]
  pub rpc_host: IpAddr,
  #[structopt(long = "rpc-port", value_name = "port", help = "Bind RPC to this port",
              default_value = "0")]
  pub rpc_port: u16,
  #[structopt(short = "h", long = "host", help = "Bind to this IPv4 address",
              default_value = "0.0.0.0")]
  pub host: Ipv4Addr,
  #[structopt(short = "p", long = "port", help = "Bind to this port", default_value = "2004")]
  pub port: u16,
  #[structopt(long = "gs-remote", value_name = "url",
              help = "Specify one or more remote Game Server", raw(display_order = "1000"))]
  pub remote: Vec<String>,
  #[structopt(long = "gs-local", value_name = "id",
              help = "Specify one or more local Game Server", raw(display_order = "1001"))]
  pub local: Vec<u16>,
}
