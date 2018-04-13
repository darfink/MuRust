use std::str::FromStr;

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

impl Class {
  pub fn from_str(input: &str) -> Option<Self> {
    match input {
      "DW" => Some(Class::DarkWizard),
      "DK" => Some(Class::DarkKnight),
      "FE" => Some(Class::FairyElf),
      "MG" => Some(Class::MagicGladiator),
      "DL" => Some(Class::DarkLord),
      "SM" => Some(Class::SoulMaster),
      "BK" => Some(Class::BladeKnight),
      "ME" => Some(Class::MuseElf),
      _ => None,
    }
  }
}

impl From<Class> for &'static str {
  fn from(class: Class) -> Self {
    match class {
      Class::DarkWizard => "DW",
      Class::DarkKnight => "DK",
      Class::FairyElf => "FE",
      Class::MagicGladiator => "MG",
      Class::DarkLord => "DL",
      Class::SoulMaster => "SM",
      Class::BladeKnight => "BK",
      Class::MuseElf => "ME",
    }
  }
}

primitive_serialize!(Class, u8);
