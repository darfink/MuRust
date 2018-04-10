#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;
extern crate murust_connect_server as mucs;

use self::options::Options;
use std::net::{SocketAddr, SocketAddrV4};
use structopt::StructOpt;

mod logger;
mod options;

// TODO: Handle ctrl-c events
fn main() {
  // Parse any CLI arguments
  let options = Options::from_args();

  // Initialize the standard logger
  logger::StdLogger::init();

  let socket_server = SocketAddrV4::new(options.host, options.port);
  let socket_rpc = SocketAddr::new(options.rpc_host, options.rpc_port);

  // Configure the host and port options provided
  let builder = mucs::ConnectServer::builder()
    .socket_connect(socket_server)
    .socket_rpc(socket_rpc);

  // Spawn the server!
  let mut server = builder.spawn().unwrap();
  info!("RPC servicing at {}", server.uri());

  // Add any remote game servers
  for game_server in options.remote {
    if let Err(error) = server.add_game_server(&game_server) {
      error!("Failed to add game server('{}') {}", &game_server, error);
    }
  }

  // Start the connect server
  server.wait().unwrap();
}
