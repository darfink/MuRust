use cursive::Cursive;
use cursive::align::HAlign;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::view::ScrollStrategy;
use cursive::views::{LinearLayout, TextView, TextContent, SelectView, Dialog, Panel};

pub fn create(console: TextContent) -> Cursive {
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

  root.set_fps(10);
  root.set_autohide_menu(false);
  root
}

fn seconds_to_hhmmss(seconds: u64) -> String {
  format!("{:02}:{:02}:{:02}", seconds / 3600, seconds / 60, seconds % 60)
}
