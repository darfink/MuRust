use muserialize::{IntegerLE, StringFixed};
use serde::{Serialize, Serializer};
use {model, mu, typenum};

/// A Character list entry.
///
/// Used in conjunction with [CharacterList](../struct.CharacterList.html).
#[derive(Serialize, Debug)]
pub struct CharacterListEntry {
  pub slot: u8,
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
  pub padding: u8,
  #[serde(with = "IntegerLE")]
  pub level: u16,
  pub ctl: mu::CtlCode,
  #[serde(serialize_with = "serialize_class")]
  pub class: mu::Class,
  pub equipment: model::Equipment,
  pub guild: mu::GuildRole,
}

/// Serializes a class value encoded as expected by the client.
fn serialize_class<S>(class: &mu::Class, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let class = *class as u8;
  ((class << 5) | ((class & 0x08) << 1)).serialize(serializer)
}
