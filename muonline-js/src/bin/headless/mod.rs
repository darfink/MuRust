use std::{io, mem};
use std::thread::JoinHandle;
use mujs;

mod logger;

pub fn run(builder: mujs::Builder) -> JoinHandle<io::Result<()>> {
  logger::StdLogger::init();
  let (server, cancel) = builder.build();
  mem::forget(cancel);
  server.serve()
}