use error::{Error, Result};
use mapping::{self, MappableToDomain};
use murust_data_model::entities::{inventory, item, Item};
use murust_repository::*;

/// A service for item management.
pub struct ItemService {
  repo_item: ItemRepository,
  repo_item_defintion: ItemDefinitionRepository,
  repo_item_eligible_class: ItemEligibleClassRepository,
}

impl ItemService {
  /// Constructs a new item service.
  pub fn new(
    repo_item: ItemRepository,
    repo_item_defintion: ItemDefinitionRepository,
    repo_item_eligible_class: ItemEligibleClassRepository,
  ) -> Self {
    ItemService {
      repo_item,
      repo_item_defintion,
      repo_item_eligible_class,
    }
  }

  pub fn find_by_id(&self, id: item::Id) -> Result<Option<Item>> {
    self
      .repo_item
      .find_by_id(id)?
      .map_or(Ok(None), |item| self.map_item_to_entity(item).map(Some))
  }

  // TODO: How can this be implemented instead?
  pub(crate) fn find_equipment_by_character_id(
    &self,
    character_id: i32,
  ) -> Result<Vec<(i32, Item)>> {
    self
      .repo_item
      .find_equipment_by_character_id(character_id)?
      .into_iter()
      .map(|(inventory_item, item)| Ok((inventory_item.slot, self.map_item_to_entity(item)?)))
      .collect::<Result<Vec<_>>>()
  }

  // TODO: How can this be implemented instead?
  pub(crate) fn find_items_by_inventory_id(
    &self,
    inventory_id: inventory::Id,
  ) -> Result<Vec<(i32, Item)>> {
    self
      .repo_item
      .find_inventory_contents_by_id(inventory_id)?
      .into_iter()
      .map(|(inventory_item, item)| Ok((inventory_item.slot, self.map_item_to_entity(item)?)))
      .collect::<Result<Vec<_>>>()
  }

  fn map_item_to_entity(&self, item: models::Item) -> Result<Item> {
    let definition: models::ItemDefinition = self
      .repo_item_defintion
      .find_by_item_code(item.code)?
      .ok_or(Error::MissingAssociation("ItemDefinition".into()))?;

    let classes = self
      .repo_item_eligible_class
      .find_by_item_code(definition.code)?
      .into_iter()
      .map(mapping::to_character_class)
      .collect::<mapping::Result<Vec<_>>>()?;

    item
      .map_to_entity(definition.map_to_entity((classes,))?)
      .map_err(Into::into)
  }
}
