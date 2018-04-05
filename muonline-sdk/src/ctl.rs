#![allow(non_upper_case_globals)]

bitflags! {
  // TODO: Validate these
  /// A collection of all control codes.
  pub struct CtlCode: u8 {
    const None          = 0;
    const Banned        = (1 << 0);
    const LootDisable   = (1 << 1);
    const Administrator = (1 << 3);
    const Invincible    = (1 << 4);
    const Invisible     = (1 << 5);
  }
}

impl Default for CtlCode {
  fn default() -> Self { CtlCode::None }
}

bitflags_serialize!(CtlCode, u8);
