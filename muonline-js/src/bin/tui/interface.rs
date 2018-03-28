use cursive::align::HAlign;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::view::ScrollStrategy;
use cursive::views::{Dialog, LinearLayout, Panel, SelectView, TextContent, TextView};
use cursive::{CbFunc, Cursive};
use std::io;
use std::sync::mpsc::Sender;

pub struct TextUserInterface {
  ui: Cursive,
}

impl TextUserInterface {
  pub fn new(console: TextContent) -> Self {
    let mut root = Cursive::new();
    let console = TextView::new_with_content(console)
      .scroll_strategy(ScrollStrategy::StickToBottom)
      .full_screen();

    let mut info = LinearLayout::vertical();
    // TODO: Implement max clients/queue?
    let items = [
      ("Host:", "-", "host"),
      ("Port:", "-", "port"),
      ("Uptime:", "00:00:00", "uptime"),
      ("Clients:", "0", "clients"),
    ];

    for &(label, value, id) in &items {
      info.add_child(
        LinearLayout::horizontal()
          .child(TextView::new(label))
          .child(
            TextView::new(value)
              .h_align(HAlign::Right)
              .with_id(id)
              .full_width(),
          ),
      );
    }

    let servers = SelectView::new()
      .item("GameServer 1 [0/10]", 1)
      .on_submit(|s, _| s.add_layer(Dialog::around(TextView::new("FAG"))));

    let panel = LinearLayout::vertical()
      .child(Dialog::around(info).title("Join Server").full_width())
      .child(Dialog::around(servers).title("Game Servers").full_screen())
      .fixed_width(40);

    root.add_fullscreen_layer(
      LinearLayout::horizontal()
        .child(Panel::new(console).full_screen())
        .child(panel)
        .full_screen(),
    );

    root
      .menubar()
      .add_subtree("Server", MenuTree::new().leaf("Quit", |s| s.quit()))
      .add_subtree("Options", MenuTree::new().leaf("Quit", |s| s.quit()));

    root.set_fps(10);
    root.set_autohide_menu(false);
    TextUserInterface { ui: root }
  }

  pub fn run(&mut self) { self.ui.run(); }

  pub fn remote(&self) -> RemoteTextUserInterface {
    RemoteTextUserInterface::new(self.ui.cb_sink().clone())
  }

  fn refresh(tui: &mut Cursive, status: &::mujs::rpc::JoinServerStatus) {
    let items: &[(&str, &Fn() -> String)] = &[
      ("host", &|| status.host.to_string()),
      ("port", &|| status.port.to_string()),
      ("clients", &|| status.clients.to_string()),
      ("uptime", &|| Self::seconds_to_hhmmss(status.uptime)),
    ];

    for &(id, content) in items.iter() {
      tui
        .find_id::<TextView>(id)
        .expect("retrieving UI element")
        .set_content(content());
    }
  }

  fn seconds_to_hhmmss(seconds: u64) -> String {
    format!(
      "{:02}:{:02}:{:02}",
      seconds / 3600,
      seconds / 60,
      seconds % 60
    )
  }
}

pub struct RemoteTextUserInterface {
  sender: Sender<Box<CbFunc>>,
}

impl RemoteTextUserInterface {
  fn new(sender: Sender<Box<CbFunc>>) -> Self { RemoteTextUserInterface { sender } }

  pub fn refresh(&self, status: ::mujs::rpc::JoinServerStatus) -> io::Result<()> {
    self.send(move |tui: &mut Cursive| TextUserInterface::refresh(tui, &status))
  }

  fn send<F: CbFunc + 'static>(&self, closure: F) -> io::Result<()> {
    self.sender.send(Box::new(closure)).map_err(|_| {
      io::Error::new(
        io::ErrorKind::BrokenPipe,
        "RPC client error; GUI disconnected",
      )
    })
  }
}
