use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use error::Result;
use models::ItemDefinition;
use schema::item_definition::dsl;

/// A repository for item definitions.
#[derive(Clone)]
pub struct ItemDefinitionRepository {
  context: DataContextInner,
}

impl ItemDefinitionRepository {
  /// Creates a new item definition repository instance.
  pub fn new(context: &DataContext) -> Self {
    ItemDefinitionRepository {
      context: context.inner(),
    }
  }

  /// Returns an item definition by item code.
  pub fn find_by_item_code(&self, item_code: i32) -> Result<Option<ItemDefinition>> {
    dsl::item_definition
      .find(item_code)
      .first::<ItemDefinition>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }
}
