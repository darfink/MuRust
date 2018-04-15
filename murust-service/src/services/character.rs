use ItemService;
use error::{Error, Result};
use mapping::MappableToDomain;
use murust_data_model::entities::{Character, Equipment, Inventory};
use murust_data_model::types::{Class, CHARACTER_SLOTS};
use murust_repository::*;

/// A collection of possible character creation errors.
#[derive(Debug)]
pub enum CharacterCreateError {
  // InvalidClass,
  // InvalidName,
  LimitReached,
}

/// A collection of possible character deletion errors.
#[derive(Debug)]
pub enum CharacterDeleteError {
  GuildCharacter,
  Blocked,
}

/// A service for character management.
pub struct CharacterService {
  // TODO: Obviously this item business is ugly as hell
  item_service: ItemService,
  repo_items: ItemRepository,
  repo_characters: CharacterRepository,
  repo_inventory: InventoryRepository,
}

impl CharacterService {
  /// Constructs a new character service.
  pub fn new(
    item_service: ItemService,
    repo_items: ItemRepository,
    repo_characters: CharacterRepository,
    repo_inventory: InventoryRepository,
  ) -> Self {
    CharacterService {
      item_service,
      repo_items,
      repo_characters,
      repo_inventory,
    }
  }

  /// Returns an account's characters.
  pub fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Character>> {
    self
      .repo_characters
      .find_by_account_id(account_id)?
      .into_iter()
      .map(|character| self.map_character_to_entity(character))
      .collect::<Result<Vec<_>>>()
  }

  /// Returns a character by name.
  pub fn find_by_name(&self, name: &str) -> Result<Option<Character>> {
    self
      .repo_characters
      .find_by_name(name)?
      .map_or(Ok(None), |character| self.map_character_to_entity(character).map(Some))
      .map_err(Into::into)
  }

  // Creates a new character and returns it as an entity.
  pub fn create(
    &self,
    name: &str,
    class: Class,
    account_id: i32,
  ) -> Result<::std::result::Result<Character, CharacterCreateError>> {
    let mut slots_free = CHARACTER_SLOTS.rev().collect::<Vec<_>>();
    for character in self.repo_characters.find_by_account_id(account_id)? {
      slots_free.remove_item(&(character.slot as usize));
    }

    match slots_free.pop() {
      // TODO: Configurable max characters
      // TODO: Validate name
      // TODO: max class
      // TODO: starting position/world etc...
      None => return Ok(Err(CharacterCreateError::LimitReached)),
      Some(slot) => {
        let inventory = self.repo_inventory.create(8, 8)?;
        self
          .repo_characters
          .create(
            slot as i32,
            name,
            class.into(),
            0,
            0,
            0,
            inventory.id,
            account_id,
          )
          .map_err(Into::into)
          .and_then(|character| {
            character
              .map_to_entity((Equipment::default(), Inventory::new(8, 8)))
              .map_err(Into::into)
          })
          .map(Ok)
      },
    }
  }

  /// Removes a character from the underlying storage.
  pub fn delete(
    &self,
    character: Character,
  ) -> Result<::std::result::Result<(), (Character, CharacterDeleteError)>> {
    // TODO: Actually validate guild/blocked.
    // TODO: Use a transaction!
    self
      .repo_items
      .delete_equipment_by_character_id(character.id)?;
    self
      .repo_items
      .clear_inventory_by_id(character.inventory.id)?;
    self
      .repo_characters
      .delete(&character.id)?;
    self
      .repo_inventory
      .delete(character.inventory.id)?;
    Ok(Ok(()))
  }

  fn map_character_to_entity(&self, character: models::Character) -> Result<Character> {
    let equipment = self
      .item_service
      .find_equipment_by_character_id(character.id)
      .and_then(|equipment| equipment.map_to_entity(()).map_err(Into::into))?;

    let inventory = self
      .repo_inventory
      .find_by_id(character.inventory_id)?
      .ok_or_else(|| Error::MissingAssociation("Inventory".into()))
      .and_then(|inventory| self.map_inventory_to_entity(inventory))?;

    character
      .map_to_entity((equipment, inventory))
      .map_err(Into::into)
  }

  fn map_inventory_to_entity(&self, inventory: models::Inventory) -> Result<Inventory> {
    let items = self.item_service.find_items_by_inventory_id(*inventory.id)?;
    inventory.map_to_entity((items,)).map_err(Into::into)
  }
}
