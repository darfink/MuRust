#[macro_use] extern crate log;
#[macro_use] extern crate structopt;
extern crate cursive;
extern crate muonline_js as mujs;
extern crate tap;

use std::net::{Ipv4Addr, SocketAddrV4};
use structopt::StructOpt;

mod headless;
mod tui;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "mujs", about = "Mu Online Season 2 Join Server")]
struct Options {
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

fn main() {
  // Parse the options from the CLI
  let options = Options::from_args();
  let socket = SocketAddrV4::new(options.host, options.port);

  // Configure the Join Server with the supplied options
  let builder = mujs::JoinServer::builder(socket);
  let builder = options.remote.iter().fold(builder, |b, s| b.remote(*s));
  let builder = options.local.iter().fold(builder, |b, c| b.local(*c));

  let thread = match options.headless {
    false => tui::run(builder),
    true => headless::run(builder),
  };

  // TODO: Log 'console' to file?
  match thread.join() {
    Ok(Err(error)) => {
      error!("Server error; {}", error);
      debug!("<main> {:#?}", error);
    },
    Err(error) => {
      error!("Server thread error");
      debug!("<main> {:#?}", error);
    },
    _ => (),
  }
}