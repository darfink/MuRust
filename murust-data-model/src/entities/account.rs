// TODO: Security code should be a string? Can be prefixed with zeros.
#[derive(Debug, Clone)]
pub struct Account {
  pub id: i32,
  pub username: String,
  pub security_code: u32,
  pub email: String,
}
