#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;
extern crate murust_connect_server as mucs;
extern crate murust_game_server as mugs;
extern crate murust_repository;
extern crate murust_service;
extern crate tempdir;

use self::options::Options;
use murust_repository::DataContext;
use murust_service::ServiceManager;
use std::net::{SocketAddr, SocketAddrV4};
use structopt::StructOpt;
use tempdir::TempDir;

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

  let (_temp, manager) = setup_test_env();
  let config = mugs::GameServerConfig {
    id: 1,
    socket: "0.0.0.0:0".parse().unwrap(),
    maximum_players: 3,
  };
  let gs = mugs::GameServer::spawn(config, manager);
  let gs_rpc = mugs::rpc::spawn_service("0.0.0.0:0".parse().unwrap(), gs.context()).unwrap();
  server.add_game_server(gs_rpc.uri()).unwrap();

  // Start the connect server
  server.wait().unwrap();
  gs.wait().unwrap();
  gs_rpc.close();
}

fn setup_test_env() -> (TempDir, ServiceManager) {
  let tmp = TempDir::new("murust-repository").expect("creating tempdir");
  let path_buf = tmp.path().join("database.sqlite");
  let path = path_buf.to_str().expect("converting temp DB path");

  let database = DataContext::new(path).expect("creating DB");
  database
    .initialize_schema()
    .expect("creating default schema");
  database.initialize_data().expect("creating test data");

  (tmp, ServiceManager::new(database))
}
