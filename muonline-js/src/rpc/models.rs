use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinServiceStatus {
  pub clients: usize,
  pub host: Ipv4Addr,
  pub port: u16,
  pub uptime: u64,
}
