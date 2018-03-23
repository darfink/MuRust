#[macro_use] extern crate log;
#[macro_use] extern crate closet;
extern crate tokio;
extern crate muonline_protocol as protocol;
extern crate muonline_packet as mupack;
extern crate muonline_packet_codec as mucodec;
extern crate futures;

pub use ipc::Ipc;
pub use server::{Builder, JoinServer};

mod ipc;
mod server;