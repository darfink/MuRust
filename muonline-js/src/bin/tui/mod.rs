use std::io;
use std::time::{Instant, Duration};
use mujs;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::view::ScrollStrategy;
use cursive::views::{LinearLayout, TextView, TextContent, SelectView, Dialog, Panel};
use futures::Async;
use jsonrpc_client_http::HttpTransport;
use futures::Future;

mod logger;

jsonrpc_client!(pub struct JoinServiceClient {
  /// Returns the status of the Join Service.
  pub fn status(&mut self) -> RpcRequest<mujs::rpc::JoinServiceStatus>;
});

pub fn run(builder: mujs::ServerBuilder) -> io::Result<()> {
  // Setup the logging facade using a thread-safe text buffer
  let console = TextContent::new(String::default());
  logger::TuiLogger::init(console.clone());

  let mut gui = create(console.clone());
  let server = builder.spawn()?;

  // TODO: Also fix finally close server pre-exit
  let transport_handle = HttpTransport::new()
    .unwrap()
    .handle(server.uri())
    .unwrap();
  let mut client = JoinServiceClient::new(transport_handle);
  let mut status_time = Instant::now();
  let mut status_future = client.status();

  gui.set_fps(10);
  while gui.is_running() {
    gui.step();

    let now = Instant::now();

    if now.duration_since(status_time) > Duration::from_millis(900) {
      match status_future.poll() {
        Ok(Async::Ready(status)) => {
          trace!("{:#?}", status);

          status_future = client.status();
          status_time = now;

          gui.find_id::<TextView>("host")
             .expect("retrieving host element")
             .set_content(status.host.to_string());

          gui.find_id::<TextView>("port")
             .expect("retrieving port element")
             .set_content(status.port.to_string());

          gui.find_id::<TextView>("clients")
             .expect("retrieving clients element")
             .set_content(status.clients.to_string());

          gui.find_id::<TextView>("uptime")
             .expect("retrieving uptime element")
             .set_content(seconds_to_hhmmss(status.uptime));
        },
        Ok(Async::NotReady) => (),
        Err(error) => error!("RPC error; {}", error),
      }
    }
  }

  // Close the Join Server as well
  server.close()
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
    ("Host:",    "-",        "host"),
    ("Port:",    "-",        "port"),
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
