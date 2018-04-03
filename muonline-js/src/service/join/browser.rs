use futures::{stream, Future, Stream};
use jsonrpc_client_http::{HttpHandle, HttpTransport};
use mugs;
use std::io;
use std::sync::Mutex;

// TODO: Ensure Game Servers are a specific version?
jsonrpc_client!(pub struct GameServerApi {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<mugs::rpc::GameServerStatus>;
});

/// A browser of all available Game Servers.
pub struct GameServerBrowser {
  servers: Mutex<Vec<GameServerApi<HttpHandle>>>,
}

impl GameServerBrowser {
  /// Constructs a new Game Server browser.
  pub fn new<'a, S, I>(servers: S) -> io::Result<Self>
  where
    S: IntoIterator<Item = I>,
    I: AsRef<str>,
  {
    let servers = HttpTransport::new()
      .and_then(|transport| {
        servers
          .into_iter()
          .map(|uri| {
            transport
              .handle(uri.as_ref())
              .map(|handle| GameServerApi::new(handle))
          })
          .collect::<Result<Vec<_>, _>>()
      })
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;
    Ok(GameServerBrowser {
      servers: Mutex::new(servers),
    })
  }

  /// Queries all available game servers.
  pub fn query_servers(
    &self,
  ) -> impl Stream<Item = mugs::rpc::GameServerStatus, Error = io::Error> + Send {
    let mut servers = self.servers.lock().unwrap();
    let requests = servers.iter_mut().map(|server| {
      server
        .status()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
    });
    stream::futures_unordered(requests)
  }
}
