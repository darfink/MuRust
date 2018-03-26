use super::interface::RemoteTextUserInterface;
use futures::sync::oneshot;
use futures::{stream, Future, IntoFuture, Stream};
use jsonrpc_client_http::HttpTransport;
use mujs::util::CancellableService;
use std::time::Duration;
use std::{io, thread};
use tokio_core::reactor::{Core, Timeout};

mod client;

pub struct TuiRpcClient(CancellableService);

impl TuiRpcClient {
  pub fn spawn(uri: &str, tui: RemoteTextUserInterface) -> io::Result<Self> {
    let (tx, rx) = oneshot::channel();
    let uri = uri.to_string();
    let thread = thread::spawn(move || Self::serve(&uri, &tui, rx));
    Ok(TuiRpcClient(CancellableService::new(thread, tx)))
  }

  pub fn close(self) -> io::Result<()> {
    self.0.close()
  }

  fn serve(
    uri: &str,
    tui: &RemoteTextUserInterface,
    cancel: oneshot::Receiver<()>,
  ) -> io::Result<()> {
    let mut core = Core::new()?;
    let transport_handle = HttpTransport::shared(&core.handle())
      .and_then(|transport| transport.handle(uri))
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

    // TODO: Support server exits from external RPC exit.
    let client = client::JoinServiceClient::new(transport_handle);
    let handle = core.handle().clone();

    // Poll the RPC server every 0.5s for an update
    let status = stream::unfold(client, |mut client| {
      let timeout = Timeout::new(Duration::from_millis(500), &handle)
        .into_future()
        .flatten();

      // TODO: Add informative error message upon RPC server disconnect
      let request = client
        .status()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
        .join(timeout)
        .map(|(status, _)| (status, client));
      Some(request)
    }).for_each(|status| tui.refresh(status));

    let future = cancel
      .map_err(|_| io::ErrorKind::BrokenPipe.into())
      .select(status)
      .map(|(result, _)| result)
      .map_err(|(error, _)| error);

    core.run(future)
  }
}
