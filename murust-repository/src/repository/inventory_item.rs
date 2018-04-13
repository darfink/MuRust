use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use models::InventoryItem;
use schema::inventory_item::dsl;
use std::io;
use util::diesel_to_io;

/// A repository for inventory items.
#[derive(Clone)]
pub struct InventoryItemRepository {
  context: DataContextInner,
}

impl InventoryItemRepository {
  /// Creates a new inventory item repository instance.
  pub fn new(context: &DataContext) -> Self {
    InventoryItemRepository {
      context: context.clone(),
    }
  }

  /// Returns an inventory's items.
  pub fn find_by_inventory_id(&self, inventory_id: i32) -> io::Result<Vec<InventoryItem>> {
    dsl::inventory_item
      .filter(dsl::inventory_id.eq(&inventory_id))
      .get_results::<InventoryItem>(&*self.context.access())
      .map_err(diesel_to_io)
  }
}
