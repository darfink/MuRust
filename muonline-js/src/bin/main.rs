#[macro_use] extern crate log;
#[macro_use] extern crate structopt;

#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

extern crate cursive;
extern crate futures;
extern crate muonline_js as mujs;
extern crate tap;

use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr, IpAddr};
use structopt::StructOpt;

mod headless;
mod tui;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "mujs", about = "Mu Online Season 2 Join Server")]
struct Options {
  #[structopt(long = "rpc-host", value_name = "host", help = "Bind RPC to this IP address", default_value = "127.0.0.1")]
  pub rpc_host: IpAddr,
  #[structopt(long = "rpc-port", value_name = "port", help = "Bind RPC to this port", default_value = "0")]
  pub rpc_port: u16,
  #[structopt(short = "h", long = "host", help = "Bind to this IPv4 address", default_value = "0.0.0.0")]
  pub host: Ipv4Addr,
  #[structopt(short = "p", long = "port", help = "Bind to this port", default_value = "2004")]
  pub port: u16,
  #[structopt(long = "headless", help = "Disable the user interface")]
  pub headless: bool,
  #[structopt(long = "gs-remote", value_name = "host:port", help = "Specify one or more remote Game Server", raw(display_order = "1000"))]
  pub remote: Vec<SocketAddrV4>,
  #[structopt(long = "gs-local", value_name = "code", help = "Specify one or more local Game Server", raw(display_order = "1001"))]
  pub local: Vec<u16>,
}

// TODO: Handle ctrl-c events
fn main() {
  // Parse the options from the CLI
  let options = Options::from_args();

  // Configure the Join Server with the supplied options
  let builder = mujs::JoinServer::builder()
    .service(SocketAddrV4::new(options.host, options.port))
    .rpc(SocketAddr::new(options.rpc_host, options.rpc_port));
  let builder = options.remote.iter().fold(builder, |b, s| b.remote(*s));
  let builder = options.local.iter().fold(builder, |b, c| b.local(*c));

  let result = match options.headless {
    false => tui::run(builder),
    true => headless::run(builder),
  };

  // TODO: Log 'console' to file?
  if let Err(error) = result {
    error!("Server error; {}", error);
    debug!("<main> {:#?}", error);
  }
}
