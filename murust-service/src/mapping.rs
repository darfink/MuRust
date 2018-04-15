use murust_data_model::entities::*;
use murust_data_model::types::{Class, ItemCode, ItemSlot, ItemStorage, Position};
use murust_repository::models;
use num_traits::FromPrimitive;
use std::{convert::TryFrom, num::TryFromIntError};

#[derive(Debug, Fail)]
pub enum MappingError {
  #[fail(display = "An integer field could not be converted.")]
  Integer(#[cause] TryFromIntError),
  #[fail(display = "An enumerable field could not be converted.")]
  Enum,
  #[fail(display = "An inventory contained invalid slot indexes.")]
  InvalidStorage,
}

impl From<TryFromIntError> for MappingError {
  fn from(error: TryFromIntError) -> Self { MappingError::Integer(error) }
}

pub type Result<T> = ::std::result::Result<T, MappingError>;

/// An interfacing for converting persistence models to domain entities.
pub trait MappableToDomain<T> {
  type Dependencies;

  /// Tries to map a model to a domain entity.
  fn map_to_entity(self, dependencies: Self::Dependencies) -> Result<T>;
}

impl MappableToDomain<Account> for models::Account {
  type Dependencies = ();

  fn map_to_entity(self, _: Self::Dependencies) -> Result<Account> {
    Ok(Account {
      id: self.id,
      username: self.username,
      security_code: u32::try_from(self.security_code)?,
      email: self.email,
    })
  }
}

impl MappableToDomain<Item> for models::Item {
  type Dependencies = ItemDefinition;

  fn map_to_entity(self, definition: Self::Dependencies) -> Result<Item> {
    Ok(Item {
      id: *self.id,
      level: u8::try_from(self.level)?,
      durability: u8::try_from(self.durability)?,
      definition: ::std::sync::Arc::new(definition),
    })
  }
}

impl MappableToDomain<ItemDefinition> for models::ItemDefinition {
  type Dependencies = (Vec<Class>,);

  fn map_to_entity(self, (eligible_classes,): Self::Dependencies) -> Result<ItemDefinition> {
    Ok(ItemDefinition {
      code: ItemCode::from(u16::try_from(self.code)?),
      name: self.name,
      equippable_slot: self.equippable_slot.map_or(Ok(None), |slot| {
        ItemSlot::from_i32(slot).ok_or(MappingError::Enum).map(Some)
      })?,
      max_durability: u8::try_from(self.max_durability)?,
      width: u8::try_from(self.width)?,
      height: u8::try_from(self.height)?,
      drop_from_monster: self.drop_from_monster,
      drop_level: u16::try_from(self.drop_level)?,
      eligible_classes,
    })
  }
}

impl MappableToDomain<Character> for models::Character {
  type Dependencies = (Equipment, Inventory);

  fn map_to_entity(self, (equipment, inventory): Self::Dependencies) -> Result<Character> {
    Ok(Character {
      id: self.id,
      slot: u8::try_from(self.slot)?,
      name: self.name,
      level: u16::try_from(self.level)?,
      class: Class::from_str(&self.class).ok_or(MappingError::Enum)?,
      experience: self.experience as u32,
      strength: u16::try_from(self.strength)?,
      agility: u16::try_from(self.agility)?,
      vitality: u16::try_from(self.vitality)?,
      energy: u16::try_from(self.energy)?,
      command: u16::try_from(self.command)?,
      map: u8::try_from(self.map)?,
      position: Position::new(
        u8::try_from(self.position_x)?,
        u8::try_from(self.position_x)?,
      ),
      player_kills: self.player_kills,
      equipment,
      inventory,
    })
  }
}

impl MappableToDomain<Inventory> for models::Inventory {
  type Dependencies = (Vec<(i32, Item)>,);

  fn map_to_entity(self, (inventory_items,): Self::Dependencies) -> Result<Inventory> {
    let mut storage = ItemStorage::new(u8::try_from(self.width)?, u8::try_from(self.height)?);
    for (slot, item) in inventory_items {
      storage
        .add_item_at_slot(u8::try_from(slot)?, item)
        .map_err(|_| MappingError::InvalidStorage)?;
    }

    Ok(Inventory {
      id: *self.id,
      money: self.money as u32,
      storage,
    })
  }
}

impl<I: IntoIterator<Item = (i32, Item)>> MappableToDomain<Equipment> for I {
  type Dependencies = ();

  fn map_to_entity(self, _: Self::Dependencies) -> Result<Equipment> {
    let mut equipment = Equipment::default();
    for (slot, item) in self.into_iter() {
      equipment[ItemSlot::from_i32(slot).ok_or(MappingError::Enum)?] = Some(item);
    }
    Ok(equipment)
  }
}

/// Parses a character class from a string.
pub fn to_character_class(eligible: models::ItemEligibleClass) -> Result<Class> {
  Class::from_str(&eligible.class).ok_or(MappingError::Enum)
}
