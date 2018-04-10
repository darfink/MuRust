use entities::ItemDefinition;
use std::sync::Arc;
use uuid::Uuid;

/// The type of ID used by item entities.
pub type Id = Uuid;

#[derive(Debug)]
pub struct Item {
  pub id: Id,
  pub level: u8,
  pub durability: u8,
  pub definition: Arc<ItemDefinition>,
}

impl Item {
  pub fn with_definition(definition: ItemDefinition) -> Self {
    Item {
      id: Id::new_v4(),
      level: 0,
      durability: definition.max_durability,
      definition: Arc::new(definition),
    }
  }
}
