use error::{Error, Result};
use mapping::MappableEntity;
use murust_data_model::configuration::Class;
use murust_data_model::entities::{item, Item, ItemDefinition};
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

  pub fn find_by_id(&self, id: &item::Id) -> Result<Option<Item>> {
    self.repo_item.find_by_id(*id)?.map_or(Ok(None), |item| {
      let definition: models::ItemDefinition = self
        .repo_item_defintion
        .find_by_item_code(item.code)?
        .ok_or(Error::MissingAssociation("ItemDefinition".into()))?;

      let classes = self
        .repo_item_eligible_class
        .find_by_item_code(definition.code)?
        .into_iter()
        .map(|eligible| Class::try_map(eligible, ()).map_err(Into::into))
        .collect::<Result<Vec<_>>>()?;

      let item = Item::try_map(item, ItemDefinition::try_map(definition, (classes,))?)?;
      Ok(Some(item))
    })
  }
}
