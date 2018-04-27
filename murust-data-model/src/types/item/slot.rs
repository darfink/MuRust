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
    static SLOTS: [ItemSlot; ItemSlot::SIZE] = [
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
