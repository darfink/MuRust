use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use object::Inventory;
use schema::inventory::dsl;
use std::io;
use util::diesel_to_io;

/// A repository for inventories.
#[derive(Clone)]
pub struct InventoryRepository {
  context: DataContextInner,
}

impl InventoryRepository {
  /// Creates a new inventory repository instance.
  pub fn new(context: &DataContext) -> Self {
    InventoryRepository {
      context: context.clone(),
    }
  }

  /// Returns an inventory by its ID.
  pub fn find_by_id(&self, id: i32) -> io::Result<Option<Inventory>> {
    dsl::inventory
      .find(id)
      .first::<Inventory>(&*self.context.access())
      .optional()
      .map_err(diesel_to_io)
  }
}
