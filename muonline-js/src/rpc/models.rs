use std::net::Ipv4Addr;

// TODO: Name this Join Server? Should be transparent Service/Server to user.
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinServiceStatus {
  pub clients: usize,
  pub host: Ipv4Addr,
  pub port: u16,
  pub uptime: u64,
}
