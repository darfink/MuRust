use self::api::GameServerApi;
use futures::{stream, Future, IntoFuture, Stream};
use jsonrpc_client_http::{HttpHandle, HttpTransport};
use mugs::rpc::GameServerStatus;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

mod api;

/// Inner data of the server browser.
struct GameServerBrowserInner {
  states: Mutex<HashMap<u16, GameServerStatus>>,
  apis: Mutex<Vec<GameServerApi<HttpHandle>>>,
}

/// A browser of all available Game Servers.
#[derive(Clone)]
pub struct GameServerBrowser(Arc<GameServerBrowserInner>);

impl GameServerBrowser {
  /// Constructs a new Game Server browser.
  pub fn new<'a, S, I>(uris: S) -> io::Result<Self>
  where
    S: IntoIterator<Item = I>,
    I: AsRef<str>,
  {
    let apis = HttpTransport::new()
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

    Ok(GameServerBrowser(Arc::new(GameServerBrowserInner {
      states: Mutex::new(HashMap::new()),
      apis: Mutex::new(apis),
    })))
  }

  /// Queries a game server by its ID.
  pub fn query(
    &self,
    server_id: u16,
  ) -> impl Future<Item = GameServerStatus, Error = io::Error> + Send {
    match self.0.states.lock().unwrap().get(&server_id) {
      Some(state) => Ok(*state),
      None => Err(io::ErrorKind::NotFound.into()),
    }.into_future()
  }

  /// Queries all available game servers.
  pub fn query_all(&self) -> impl Stream<Item = GameServerStatus, Error = io::Error> + Send {
    let mut apis = self.0.apis.lock().unwrap();
    let requests = apis.iter_mut().map(|server| {
      server
        .status()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
    });

    let this = self.clone();
    stream::futures_unordered(requests).inspect(move |state| {
      // Update the local state for each server upon any query
      let _ = this.0.states.lock().unwrap().insert(state.id, *state);
    })
  }
}
