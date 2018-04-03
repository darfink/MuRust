use mugs;
use std::io;
use traits::QueryableGameServer;

/// A Game Server option.
pub enum GameServerOption {
  Remote(String),
  Local(mugs::ServerBuilder),
}

/// An implementation for remote servers.
impl QueryableGameServer for String {
  fn uri(&self) -> &str { self.as_ref() }
}

/// An implementation for local servers.
impl QueryableGameServer for mugs::GameServer {
  fn uri(&self) -> &str { self.uri() }
}

/// Returns registered servers as a Queryable collection.
pub fn spawn(servers: Vec<GameServerOption>) -> io::Result<Vec<Box<QueryableGameServer>>> {
  servers
    .into_iter()
    .map(|option| match option {
      // Remote server's are assumed to already have been spawned
      GameServerOption::Remote(string) => Ok(Box::new(string) as Box<QueryableGameServer>),
      // Local game servers must be spawned and managed
      GameServerOption::Local(builder) => builder
        .spawn()
        .map(|s| Box::new(s) as Box<QueryableGameServer>),
    })
    .collect::<Result<Vec<Box<QueryableGameServer>>, _>>()
}
