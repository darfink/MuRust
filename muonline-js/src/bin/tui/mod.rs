use self::interface::TextUserInterface;
use self::rpc::TuiRpcClient;
use cursive::views::TextContent;
use mujs;
use std::io;

mod interface;
mod logger;
mod rpc;

pub fn run(builder: mujs::ServerBuilder) -> io::Result<()> {
  // Setup the logging facade using a thread-safe text buffer
  let console = TextContent::new(String::default());
  logger::TuiLogger::init(console.clone());

  let mut tui = TextUserInterface::new(console.clone());
  let join_server = builder.spawn()?;
  let rpc_client = TuiRpcClient::spawn(join_server.uri(), tui.remote())?;

  // TODO: This should be inside main()
  info!("RPC servicing at {}", join_server.uri());

  tui.run();
  rpc_client.close()?;
  join_server.close()
}
