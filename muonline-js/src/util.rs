use futures::sync::oneshot;
use std::{io, thread::JoinHandle};

pub struct CancellableService {
  thread: JoinHandle<io::Result<()>>,
  cancel: oneshot::Sender<()>,
}

impl CancellableService {
  pub fn new(thread: JoinHandle<io::Result<()>>, cancel: oneshot::Sender<()>) -> Self {
    CancellableService { thread, cancel }
  }

  pub fn wait(self) -> io::Result<()> {
    Self::join_thread(self.thread)
  }

  pub fn close(self) -> io::Result<()> {
    self
      .cancel
      .send(())
      .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "service already closed"))?;
    Self::join_thread(self.thread)
  }

  fn join_thread(thread: JoinHandle<io::Result<()>>) -> io::Result<()> {
    thread
      .join()
      .map_err(|any| {
        let error = any.downcast_ref::<io::Error>().unwrap();
        io::Error::new(error.kind(), error.to_string())
      })
      .and_then(|r| r)
  }
}
