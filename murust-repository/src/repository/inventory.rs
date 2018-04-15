use boolinator::Boolinator;
use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use error::Result;
use models::Inventory;
use schema::inventory::dsl;
use types::UuidWrapper;

/// A repository for inventories.
#[derive(Clone)]
pub struct InventoryRepository {
  context: DataContextInner,
}

impl InventoryRepository {
  /// Creates a new inventory repository instance.
  pub fn new(context: &DataContext) -> Self {
    InventoryRepository {
      context: context.inner(),
    }
  }

  /// Returns an inventory by its ID.
  pub fn find_by_id<I: Into<UuidWrapper>>(&self, id: I) -> Result<Option<Inventory>> {
    dsl::inventory
      .find(&id.into())
      .first::<Inventory>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Saves an inventory by inserting or replacing it.
  pub fn save(&self, inventory: &Inventory) -> Result<()> {
    diesel::replace_into(dsl::inventory)
      .values(inventory)
      .execute(&*self.context.access())?;
    Ok(())
  }

  /// Deletes an inventory by its ID.
  pub fn delete<I: Into<UuidWrapper>>(&self, inventory_id: I) -> Result<()> {
    diesel::delete(dsl::inventory.filter(dsl::id.eq(&inventory_id.into())))
      .execute(&*self.context.access())
      .and_then(|count| (count == 1).ok_or(diesel::result::Error::NotFound))
      .map_err(Into::into)
  }
}
