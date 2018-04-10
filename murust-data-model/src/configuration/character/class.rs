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

impl FromStr for Class {
  type Err = &'static str;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "DW" => Ok(Class::DarkWizard),
      "DK" => Ok(Class::DarkKnight),
      "FE" => Ok(Class::FairyElf),
      "MG" => Ok(Class::MagicGladiator),
      "DL" => Ok(Class::DarkLord),
      "SM" => Ok(Class::SoulMaster),
      "BK" => Ok(Class::BladeKnight),
      "ME" => Ok(Class::MuseElf),
      _ => Err("invalid class acronym"),
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
