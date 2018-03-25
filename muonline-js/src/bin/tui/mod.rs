use std::io;
use cursive::views::TextContent;
use mujs;

mod interface;
mod logger;
mod rpc;

pub fn run(builder: mujs::ServerBuilder) -> io::Result<()> {
  // Setup the logging facade using a thread-safe text buffer
  let console = TextContent::new(String::default());
  logger::TuiLogger::init(console.clone());

  let mut gui = interface::create(console.clone());

  let join_server = builder.spawn()?;
  let rpc_client = rpc::TuiRpcClient::spawn(join_server.uri(), gui.cb_sink().clone())?;

  gui.run();
  rpc_client.close()?;
  join_server.close()
}
