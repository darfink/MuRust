/// A collection of all character classes.
#[repr(u8)]
#[derive(Primitive, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum Class {
  DarkWizard = 0,
  DarkKnight = 1,
  FairyElf = 2,
  MagicGladiator = 3,
  DarkLord = 4,
  SoulMaster = 8,
  BladeKnight = 9,
  MuseElf = 10,
}

impl Default for Class {
  fn default() -> Self { Class::DarkWizard }
}

primitive_serialize!(Class, u8);
