use std::net::SocketAddr;
use std::sync::{Arc, mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use cursive::{Cursive, CbFunc, views::TextView};
use tap::TapResultOps;
use mujs::Ipc;

/// Text user interface communicator
#[derive(Clone)]
pub struct TuiIpc {
  sender: Sender<Box<CbFunc>>,
  clients: Arc<AtomicUsize>,
}

impl TuiIpc {
  pub fn new(sender: Sender<Box<CbFunc>>) -> Self {
    TuiIpc { sender, clients: Arc::new(AtomicUsize::new(0)) }
  }

  fn send<F: CbFunc + 'static>(&self, closure: F) {
    let _ = self.sender.send(Box::new(closure))
      .tap_err(|error| error!("GUI communication error {}", error));
  }

  fn update_clients(&self, clients: usize) {
    self.send(move |s: &mut Cursive| {
      s.find_id::<TextView>("clients")
        .expect("retrieving clients element")
        .set_content(clients.to_string());
    });
  }
}

impl Ipc for TuiIpc {
  fn on_connect(&self, _: SocketAddr) {
    self.update_clients(self.clients.fetch_add(1, Ordering::SeqCst) + 1);
  }

  fn on_disconnect(&self, _: SocketAddr) {
    self.update_clients(self.clients.fetch_sub(1, Ordering::SeqCst) - 1);
  }

  fn on_exit(&self) {
    // TODO: Should the GUI quit instantly when the server exits?
    self.send(|s: &mut Cursive| s.quit());
  }
}

// TODO: Why is this required?
unsafe impl Sync for TuiIpc { }