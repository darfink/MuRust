use ItemService;
use error::{Error, Result};
use mapping::MappableToDomain;
use murust_data_model::entities::{Character, Inventory};
use murust_repository::*;

/// A service for character management.
pub struct CharacterService {
  item_service: ItemService,
  repo_characters: CharacterRepository,
  repo_inventory: InventoryRepository,
}

impl CharacterService {
  /// Constructs a new character service.
  pub fn new(
    item_service: ItemService,
    repo_characters: CharacterRepository,
    repo_inventory: InventoryRepository,
  ) -> Self {
    CharacterService {
      item_service,
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
    let items = self.item_service.find_items_by_inventory_id(inventory.id)?;
    inventory.map_to_entity((items,)).map_err(Into::into)
  }
}
