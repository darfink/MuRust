use std::{io, thread, thread::JoinHandle};
use std::time::Duration;
use std::sync::mpsc::Sender;
use cursive::CbFunc;
use futures::{Future, Stream, IntoFuture, stream};
use futures::sync::oneshot;
use jsonrpc_client_http::HttpTransport;
use tokio_core::reactor::{Core, Timeout};

mod client;

pub struct TuiRpcClient {
  thread: JoinHandle<io::Result<()>>,
  cancel: oneshot::Sender<()>,
}

impl TuiRpcClient {
  pub fn spawn(uri: &str, gui: Sender<Box<CbFunc>>) -> io::Result<Self> {
    let (tx, rx) = oneshot::channel();
    let uri = uri.to_string();
    let thread = thread::spawn(move || Self::serve(uri, gui, rx));

    Ok(TuiRpcClient {
      thread,
      cancel: tx,
    })
  }

  pub fn wait(self) -> io::Result<()> {
    Self::join_thread(self.thread)
  }

  pub fn close(self) -> io::Result<()> {
    self.cancel.send(())
      .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "service already closed"))?;
    Self::join_thread(self.thread)
  }

  fn join_thread(thread: JoinHandle<io::Result<()>>) -> io::Result<()> {
    thread.join()
      .map_err(|any| {
        let error = any.downcast_ref::<io::Error>().unwrap();
        io::Error::new(error.kind(), error.to_string())
      })
      .and_then(|r| r)
  }

  fn serve(
      uri: String,
      gui: Sender<Box<CbFunc>>,
      cancel: oneshot::Receiver<()>,
  ) -> io::Result<()> {
    let mut core = Core::new()?;
    let transport_handle = HttpTransport::shared(&core.handle())
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?
      .handle(&uri)
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

    // TODO: Support server exits from external RPC exit.
    let client = client::JoinServiceClient::new(transport_handle);
    let handle = core.handle().clone();

    // Poll the RPC server every 0.9s for an update
    let status = stream::unfold(client, |mut client| {
      let timeout = Timeout::new(Duration::from_millis(900), &handle)
        .into_future()
        .flatten();

      let request = client.status()
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
        .join(timeout)
        .and_then(|(status, _)| Ok((status, client)));

      Some(request)
    }).for_each(|status| {
      gui
        .send(Box::new(|gui: &mut ::cursive::Cursive| super::interface::refresh(gui, status)))
        .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "RPC client error; GUI disconnected"))
    });

    let future = cancel
      .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "RPC client error; Front-end disconnected"))
      .select(status)
      .map(|(result, _)| result)
      .map_err(|(error, _)| error);

    core.run(future)
  }
}
