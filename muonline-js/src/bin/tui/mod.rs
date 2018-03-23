use std::io;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
use mujs;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::view::ScrollStrategy;
use cursive::views::{LinearLayout, TextView, TextContent, SelectView, Dialog, Panel};

mod ipc;
mod logger;

pub fn run(builder: mujs::Builder) -> JoinHandle<io::Result<()>> {
  // Setup the logging facade using a thread-safe text buffer
  let console = TextContent::new(String::default());
  logger::TuiLogger::init(console.clone());

  let mut gui = create(console.clone());
  let tui_ipc = ipc::TuiIpc::new(gui.cb_sink().clone());
  let (server, cancel) = builder.ipc(Arc::new(tui_ipc)).build();
  let thread = server.serve();

  let boot = Instant::now();
  let mut last_uptime = 0;

  gui.set_fps(10);
  while gui.is_running() {
    gui.step();

    // TODO: Uptime start after port has been bound?
    let uptime = Instant::now().duration_since(boot).as_secs();
    if uptime != last_uptime {
      gui.find_id::<TextView>("uptime")
        .expect("retrieving uptime element")
        .set_content(seconds_to_hhmmss(uptime));
      last_uptime = uptime;
    }
  }

  // If this fails, the server has already been stopped
  let _ = cancel.send(());
  thread
}

fn create(console: TextContent) -> Cursive {
  let mut root = Cursive::new();

  let console = TextView::new_with_content(console)
    .scroll_strategy(ScrollStrategy::StickToBottom)
    .full_screen();

  let mut info = LinearLayout::vertical();
  // TODO: Implement max clients/queue?
  // TODO: Dynamic update host & port.
  let items = [
    ("Host:",    "0.0.0.0",  "host"),
    ("Port:",    "2004",     "port"),
    ("Uptime:",  "00:00:00", "uptime"),
    ("Clients:", "0",        "clients"),
  ];

  for &(label, value, id) in &items {
    info.add_child(LinearLayout::horizontal()
      .child(TextView::new(label))
      .child(TextView::new(value)
        .h_align(HAlign::Right)
        .with_id(id)
        .full_width())
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
      .full_screen()
  );

  root.menubar()
    .add_subtree("Server", MenuTree::new().leaf("Quit", |s| s.quit()))
    .add_subtree("Options", MenuTree::new().leaf("Quit", |s| s.quit()));

  root.set_autohide_menu(false);
  root
}

fn seconds_to_hhmmss(seconds: u64) -> String {
  format!("{:02}:{:02}:{:02}", seconds / 3600, seconds / 60, seconds % 60)
}