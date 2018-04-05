pub use self::code::ItemCode;
pub use self::group::ItemGroup;
use std::ops::Deref;

mod code;
mod group;

/// An item identifier.
///
/// This also contains an item modifer due to how the client associates item's
/// with identifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ItemId {
  code: ItemCode,
  modifier: u8,
}

impl ItemId {
  /// Constructs a new item identifier.
  pub fn new<I: Into<ItemCode>>(code: I) -> Self {
    ItemId {
      code: code.into(),
      modifier: 0,
    }
  }

  /// Constructs a new item identifier with a modifier.
  pub fn with_modifier<I: Into<ItemCode>>(code: I, modifier: u8) -> Self {
    ItemId {
      code: code.into(),
      modifier,
    }
  }

  /// Returns the item's modifier.
  pub fn modifier(&self) -> u8 { self.modifier }
}

impl Deref for ItemId {
  type Target = ItemCode;

  /// Returns the underlying item code representation.
  fn deref(&self) -> &ItemCode { &self.code }
}
