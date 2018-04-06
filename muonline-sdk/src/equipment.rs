/// A collection of all character equipment slots.
#[repr(u8)]
#[derive(Primitive, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
pub enum Slot {
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
  /* TODO: USE ZEN HERE REALLY?
   * Zen, */
}

primitive_serialize!(Slot, u8);

impl Slot {
  pub fn values() -> ::std::slice::Iter<'static, Slot> {
    static SLOTS: [Slot; 12] = [
      Slot::WeaponRight,
      Slot::WeaponLeft,
      Slot::Helm,
      Slot::Armor,
      Slot::Pants,
      Slot::Gloves,
      Slot::Boots,
      Slot::Wings,
      Slot::Helper,
      Slot::Amulet,
      Slot::RingRight,
      Slot::RingLeft,
    ];
    SLOTS.into_iter()
  }
}

// TODO: This should be 108 total?
// pub enum PlayerInventory {
//    Equipment(Slot),
//    Zen,
//    Inventory,
//    Shop,
//}
