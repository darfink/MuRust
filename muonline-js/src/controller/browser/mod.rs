use self::client::GameServerApi;
use futures::{stream, Future, Stream};
use jsonrpc_client_http::{HttpHandle, HttpTransport};
use mugs;
use std::io;
use std::sync::Mutex;

mod client;

/// A browser of all available Game Servers.
pub struct GameServerBrowser {
  servers: Mutex<Vec<GameServerApi<HttpHandle>>>,
}

impl GameServerBrowser {
  /// Constructs a new Game Server browser.
  pub fn new<'a, S, I>(uris: S) -> io::Result<Self>
  where
    S: IntoIterator<Item = I>,
    I: AsRef<str>,
  {
    let servers = HttpTransport::new()
      .and_then(|transport| {
        let uri_to_api = |uri: I| {
          transport
            .handle(uri.as_ref())
            .map(|handle| GameServerApi::new(handle))
        };
        uris
          .into_iter()
          .map(uri_to_api)
          .collect::<Result<Vec<_>, _>>()
      })
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;
    let servers = Mutex::new(servers);
    Ok(GameServerBrowser { servers })
  }

  /// Queries all available game servers.
  pub fn query_all(
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
