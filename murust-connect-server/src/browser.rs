use futures::{stream, Future, IntoFuture, Stream};
use jsonrpc_client_http::{HttpHandle, HttpTransport};
use std::sync::{Arc, Mutex, MutexGuard};
use std::{io, collections::HashMap, net::Ipv4Addr};

/// Inner data of the server browser.
struct GameServerBrowserInner {
  // TODO: Transport does not need to be within a Mutex.
  transport: HttpTransport,
  states: HashMap<GameServerId, GameServerStatus>,
  apis: Vec<GameServerApi<HttpHandle>>,
}

/// A browser of all available Game Servers.
#[derive(Clone)]
pub struct GameServerBrowser(Arc<Mutex<GameServerBrowserInner>>);

impl GameServerBrowser {
  /// Constructs a new Game Server browser.
  pub fn new() -> io::Result<Self> {
    Ok(GameServerBrowser(Arc::new(Mutex::new(
      GameServerBrowserInner {
        transport: HttpTransport::new()
          .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?,
        states: HashMap::new(),
        apis: Vec::new(),
      },
    ))))
  }

  /// Adds a remote game server to the browser.
  pub fn add(&self, server_uri: &str) -> io::Result<()> {
    let mut inner = self.inner();
    let api_client = inner
      .transport
      .handle(server_uri)
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
      .map(|client| GameServerApi::new(client))?;
    inner.apis.push(api_client);
    Ok(())
  }

  /// Queries a game server by its ID.
  pub fn query(
    &self,
    server_id: GameServerId,
  ) -> impl Future<Item = GameServerStatus, Error = io::Error> + Send {
    match self.inner().states.get(&server_id) {
      Some(state) => Ok(*state),
      None => Err(io::ErrorKind::NotFound.into()),
    }.into_future()
  }

  /// Queries all available game servers.
  pub fn query_all(&self) -> impl Stream<Item = GameServerStatus, Error = io::Error> + Send {
    let mut inner = self.inner();
    let requests = inner.apis.iter_mut().map(|server| {
      server
        .status()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
    });
    let requests = stream::futures_unordered(requests);

    let this = self.clone();
    requests.inspect(move |state| {
      // Update the local state for each server upon any query
      this.inner().states.insert(state.id, *state);
    })
  }

  /// Returns the inner context.
  fn inner<'a>(&'a self) -> MutexGuard<'a, GameServerBrowserInner> { self.0.lock().unwrap() }
}

/// A Game Server ID type.
pub type GameServerId = u16;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameServerStatus {
  pub id: u16,
  pub host: Ipv4Addr,
  pub port: u16,
  pub clients: usize,
  pub max_clients: usize,
  pub uptime: u64,
}

impl GameServerStatus {
  pub fn load_factor(&self) -> f32 { (self.clients as f32) / (self.max_clients as f32) }
}

jsonrpc_client!(pub struct GameServerApi {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<GameServerStatus>;
});
