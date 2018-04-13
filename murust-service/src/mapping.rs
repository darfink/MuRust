use murust_data_model::configuration::{Class, ItemCode, ItemSlot};
use murust_data_model::entities::{Account, Item, ItemDefinition};
use murust_repository::models;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::num::TryFromIntError;

#[derive(Debug, Fail)]
pub enum MappingError {
  #[fail(display = "An integer field could not be converted.")]
  Integer(#[cause] TryFromIntError),
  #[fail(display = "An enumerable field could not be converted.")]
  Enum,
}

impl From<TryFromIntError> for MappingError {
  fn from(error: TryFromIntError) -> Self { MappingError::Integer(error) }
}

/// An interfacing for converting persistence models to domain entities.
pub trait MappableEntity: Sized {
  type Model;
  type Deps;

  fn try_map(model: Self::Model, deps: Self::Deps) -> Result<Self, MappingError>;
}

impl MappableEntity for Account {
  type Model = models::Account;
  type Deps = ();

  fn try_map(account: Self::Model, _: Self::Deps) -> Result<Self, MappingError> {
    Ok(Account {
      id: account.id,
      username: account.username,
      security_code: u32::try_from(account.security_code)?,
      email: account.email,
    })
  }
}

impl MappableEntity for Item {
  type Model = models::Item;
  type Deps = ItemDefinition;

  fn try_map(item: Self::Model, definition: Self::Deps) -> Result<Self, MappingError> {
    Ok(Item {
      id: *item.id,
      level: u8::try_from(item.level)?,
      durability: u8::try_from(item.durability)?,
      definition: ::std::sync::Arc::new(definition),
    })
  }
}

impl MappableEntity for ItemDefinition {
  type Model = models::ItemDefinition;
  type Deps = (Vec<Class>,);

  fn try_map(
    definition: Self::Model,
    (eligible_classes,): Self::Deps,
  ) -> Result<Self, MappingError> {
    Ok(ItemDefinition {
      code: ItemCode::from(u16::try_from(definition.code)?),
      name: definition.name,
      equippable_slot: definition.equippable_slot.map_or(Ok(None), |slot| {
        ItemSlot::from_i32(slot).ok_or(MappingError::Enum).map(Some)
      })?,
      max_durability: u8::try_from(definition.max_durability)?,
      width: u8::try_from(definition.width)?,
      height: u8::try_from(definition.height)?,
      drop_from_monster: definition.drop_from_monster,
      drop_level: u16::try_from(definition.drop_level)?,
      eligible_classes,
    })
  }
}

impl MappableEntity for Class {
  type Model = models::ItemEligibleClass;
  type Deps = ();

  fn try_map(eligible: Self::Model, _: Self::Deps) -> Result<Self, MappingError> {
    Class::from_str(&eligible.class).ok_or(MappingError::Enum)
  }
}
