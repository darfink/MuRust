use game::client::CharacterMove;
use murust_data_model::types::{Direction, Position};
use num_traits::FromPrimitive;
use serde::{de, de::{SeqAccess, Visitor}};
use std::fmt;

pub struct CharacterMoveVisitor;

impl<'de> Visitor<'de> for CharacterMoveVisitor {
  type Value = CharacterMove;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("character movement packet")
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    let mut position = Position::new(
      seq
        .next_element::<u8>()?
        .ok_or(de::Error::custom("x field missing"))?,
      seq
        .next_element::<u8>()?
        .ok_or(de::Error::custom("y field missing"))?,
    );

    let meta = seq
      .next_element::<u8>()?
      .ok_or(de::Error::custom("meta field missing"))?;
    let direction =
      Direction::from_u8(meta >> 4).ok_or(de::Error::custom("invalid direction value"))?;

    let mut path_count = (meta & 0xF).min(15) as usize;
    let mut path = Vec::with_capacity(path_count);
    path.push(position);

    for _ in 0..((path_count + 1) / 2) {
      let entry = seq
        .next_element::<u8>()?
        .ok_or(de::Error::custom("path entry missing"))?;

      for &dir in [(entry >> 4), (entry & 0xF)].iter() {
        let direction = Direction::from_u8(dir).ok_or(de::Error::custom("invalid path entry"))?;
        position = position.advance(direction);
        path.push(position);

        path_count -= 1;
        if path_count == 0 {
          break;
        }
      }
    }

    Ok(CharacterMove { direction, path })
  }
}
