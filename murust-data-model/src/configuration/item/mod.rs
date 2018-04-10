pub use self::code::ItemCode;
pub use self::group::ItemGroup;
pub use self::slot::ItemSlot;
pub use self::storage::ItemStorage;
use std::ops::Deref;

mod code;
mod group;
mod slot;
mod storage;

/// An item type identifier.
///
/// This also contains an item modifer due to how the client associates item's
/// with identifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ItemTypeId {
  code: ItemCode,
  modifier: u8,
}

impl ItemTypeId {
  /// Constructs a new item identifier.
  pub fn new<I: Into<ItemCode>>(code: I) -> Self {
    ItemTypeId {
      code: code.into(),
      modifier: 0,
    }
  }

  /// Constructs a new item identifier with a modifier.
  pub fn with_modifier<I: Into<ItemCode>>(code: I, modifier: u8) -> Self {
    ItemTypeId {
      code: code.into(),
      modifier,
    }
  }

  /// Returns the item's modifier.
  pub fn modifier(&self) -> u8 { self.modifier }
}

impl Deref for ItemTypeId {
  type Target = ItemCode;

  /// Returns the underlying item code representation.
  fn deref(&self) -> &ItemCode { &self.code }
}
