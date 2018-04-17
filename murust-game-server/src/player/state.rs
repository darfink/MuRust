use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayerState {
  // Initial,
  LoginScreen,
  Authenticated,
  CharacterSelection,
  Teleporting,
  Dead,
}

impl PlayerState {
  pub fn try_advance_to(&mut self, other: PlayerState) -> bool {
    *self = other;
    true
  }
}

impl fmt::Display for PlayerState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        PlayerState::LoginScreen => "Login Screen",
        PlayerState::Authenticated => "Authenticated",
        PlayerState::CharacterSelection => "Character Selection",
        PlayerState::Teleporting => "Teleporting",
        PlayerState::Dead => "Dead",
      }
    )
  }
}
