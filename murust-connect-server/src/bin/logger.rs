use log;
use std::io::{self, Write};

pub struct StdLogger;

impl StdLogger {
  pub fn init() {
    log::set_logger(&LOGGER).expect("initializing default logger");
    log::set_max_level(log::LevelFilter::Info);
  }
}

impl log::Log for StdLogger {
  // TODO: Make this generic for all loggers
  fn enabled(&self, meta: &log::Metadata) -> bool { meta.target().starts_with("mu") }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      // TODO: Cleanup the duplicate formatting
      let _ = match record.level() {
        log::Level::Error => {
          io::stderr().write_fmt(format_args!("[{}]: {}\n", record.level(), record.args()))
        },
        _ => io::stdout().write_fmt(format_args!("[{}]: {}\n", record.level(), record.args())),
      };
    }
  }

  fn flush(&self) {
    // The process is probably detached in case this fails
    let _ = io::stdout().flush();
  }
}

static LOGGER: StdLogger = StdLogger;
