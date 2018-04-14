/// A collection of all item groups.
#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ItemGroup {
  Sword = 0,
  Axe = 1,
  Mace = 2,
  Spear = 3,
  Bow = 4,
  Staff = 5,
  Shield = 6,
  Helm = 7,
  Armor = 8,
  Pants = 9,
  Gloves = 10,
  Boots = 11,
  // Contains primarily wings & orbs.
  Wings = 12,
  // Contains primarily minions & ornaments.
  Helper = 13,
  // Contains primarily consumables.
  Potion = 14,
  // Contains scrolls and parchments.
  Scroll = 15,
}

primitive_serialize!(ItemGroup, u8);

impl ItemGroup {
  /// The size of each item group.
  pub const GROUP_SIZE: u16 = (1 << 9);
}
