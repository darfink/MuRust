use log;
use std::cell::UnsafeCell;
use cursive::views::TextContent;

pub struct TuiLogger {
  // `TextContent` has thread-safe interior mutability
  buffer: UnsafeCell<TextContent>,
}

impl TuiLogger {
  /// Initializes a new text user interface logger.
  pub fn init(buffer: TextContent) {
    let this = TuiLogger { buffer: UnsafeCell::new(buffer) };
    log::set_boxed_logger(Box::new(this)).expect("initializing standard logger");
    log::set_max_level(log::LevelFilter::Info);
  }
}

impl log::Log for TuiLogger {
  fn enabled(&self, meta: &log::Metadata) -> bool {
    meta.target().starts_with("mu")
  }

  fn log(&self, record: &log::Record) {
    // TODO: Cannot have infinite history, set maximum
    if self.enabled(record.metadata()) {
      unsafe {
        (*self.buffer.get()).append(format!("[{}]: {}\n", record.level(), record.args()));
      }
    }
  }

  fn flush(&self) { }
}

unsafe impl Sync for TuiLogger {}