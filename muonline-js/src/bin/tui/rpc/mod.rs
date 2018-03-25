use std::{io, thread, thread::JoinHandle};
use std::sync::mpsc::Sender;
use cursive::CbFunc;
use futures::{Future, future, future::Loop};
use futures::sync::oneshot;
use jsonrpc_client_http::{HttpTransport, HttpHandle};
use tokio;

mod client;

pub struct TuiRpcClient {
  thread: JoinHandle<io::Result<()>>,
  cancel: oneshot::Sender<()>,
}

impl TuiRpcClient {
  pub fn spawn(uri: &str, gui: Sender<Box<CbFunc>>) -> io::Result<Self> {
    let runtime = tokio::runtime::Runtime::new()?;
    let handle = HttpTransport::shared(runtime.handle())
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?
      .handle(uri)
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

    let (tx, rx) = oneshot::channel();
    let thread = thread::spawn(move || Self::serve(runtime, handle, gui, rx));

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
      mut runtime: tokio::runtime::Runtime,
      handle: HttpHandle,
      _gui: Sender<Box<CbFunc>>,
      cancel: oneshot::Receiver<()>,
  ) -> io::Result<()> {
    // TODO: Support server exits from external RPC exit.
    let client = client::JoinServiceClient::new(handle);

    let status = future::loop_fn(client, |mut client| {
      client.status()
        .and_then(|status| {
          info!("YOYOYOYO: {:#?}", status);
          Ok(Loop::Continue(client))
        })
        .or_else(|error| {
          error!("RPC client error; {}", error);
          Ok(Loop::Break(()))
        })
    })
    .map_err(|error: ()| error);

    let future = cancel
      .map_err(|_| error!("RPC client error; GUI disconnected"))
      .select(status)
      .map(|_| ())
      .map_err(|(error, _)| error);

    runtime.spawn(future);
    runtime.shutdown_on_idle().wait().unwrap();
    Ok(())
  }
}
