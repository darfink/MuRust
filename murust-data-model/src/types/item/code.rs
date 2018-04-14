use num_traits::FromPrimitive;
use types::ItemGroup;

/// An item code.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct ItemCode(u16);

impl ItemCode {
  /// Creates a new item code from an index and group.
  pub fn new(group: ItemGroup, index: u16) -> Self {
    ItemCode(group as u16 * ItemGroup::GROUP_SIZE + index)
  }

  /// Returns the item's associated group.
  pub fn group(&self) -> ItemGroup {
    ItemGroup::from_u8((self.0 / ItemGroup::GROUP_SIZE) as u8).expect("validating item group")
  }

  /// Returns the item's group index.
  pub fn index(&self) -> u16 { self.0 % ItemGroup::GROUP_SIZE }

  /// Returns the underlying code.
  pub fn as_raw(&self) -> u16 { self.0 }

  /// Returns the group and index as a tuple.
  pub fn tuple(&self) -> (ItemGroup, u16) { (self.group(), self.index()) }
}

impl From<u16> for ItemCode {
  fn from(code: u16) -> Self { ItemCode(code) }
}

impl From<(ItemGroup, u16)> for ItemCode {
  fn from((group, index): (ItemGroup, u16)) -> Self { ItemCode::new(group, index) }
}
