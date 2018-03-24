use std::io;
use mujs;

mod logger;

pub fn run(builder: mujs::ServerBuilder) -> io::Result<()> {
  logger::StdLogger::init();
  builder.spawn().and_then(|server| server.wait())
}
