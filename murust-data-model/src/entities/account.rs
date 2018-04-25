use entities::Character;

// TODO: Security code should be a string? Can be prefixed with zeros.
// TODO: Include characters here as well
// TODO: Add vault ID
#[derive(Debug)]
pub struct Account {
  pub id: i32,
  pub username: String,
  pub security_code: u32,
  pub email: String,
  pub characters: Vec<Character>,
}
