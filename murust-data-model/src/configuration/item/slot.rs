use enum_map;
use num_traits::FromPrimitive;

/// A collection of all possible item slots.
#[repr(u8)]
#[derive(Primitive, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum ItemSlot {
  WeaponRight = 0,
  WeaponLeft = 1,
  Helm = 2,
  Armor = 3,
  Pants = 4,
  Gloves = 5,
  Boots = 6,
  Wings = 7,
  Helper = 8,
  Amulet = 9,
  RingRight = 10,
  RingLeft = 11,
}

primitive_serialize!(ItemSlot, u8);

impl ItemSlot {
  pub const SIZE: usize = 12;

  /// Returns all possible item slot values.
  pub fn values() -> ::std::slice::Iter<'static, ItemSlot> {
    static SLOTS: [ItemSlot; 12] = [
      ItemSlot::WeaponRight,
      ItemSlot::WeaponLeft,
      ItemSlot::Helm,
      ItemSlot::Armor,
      ItemSlot::Pants,
      ItemSlot::Gloves,
      ItemSlot::Boots,
      ItemSlot::Wings,
      ItemSlot::Helper,
      ItemSlot::Amulet,
      ItemSlot::RingRight,
      ItemSlot::RingLeft,
    ];
    SLOTS.into_iter()
  }
}

impl<V> enum_map::Internal<V> for ItemSlot {
  /// An array of all values
  type Array = [V; Self::SIZE];

  /// Converts the array to a slice.
  fn slice(array: &Self::Array) -> &[V] { array }

  /// Converts the array to a mutable slice.
  fn slice_mut(array: &mut Self::Array) -> &mut [V] { array }

  /// Return's an item slot from an index.
  fn from_usize(value: usize) -> Self { <Self as FromPrimitive>::from_usize(value).unwrap() }

  /// Return's an item slot's index.
  fn to_usize(self) -> usize { self as usize }

  /// Return's an item slot collection through a function.
  fn from_function<F: FnMut(Self) -> V>(mut function: F) -> Self::Array {
    [
      function(ItemSlot::WeaponRight),
      function(ItemSlot::WeaponLeft),
      function(ItemSlot::Helm),
      function(ItemSlot::Armor),
      function(ItemSlot::Pants),
      function(ItemSlot::Gloves),
      function(ItemSlot::Boots),
      function(ItemSlot::Wings),
      function(ItemSlot::Helper),
      function(ItemSlot::Amulet),
      function(ItemSlot::RingRight),
      function(ItemSlot::RingLeft),
    ]
  }
}

// TODO: This should be 108 total?
// pub enum PlayerInventory {
//    Equipment(Slot),
//    Zen,
//    Inventory,
//    Shop,
//}
