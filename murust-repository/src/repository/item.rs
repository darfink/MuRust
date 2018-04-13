use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use error::Result;
use models::{EquipmentItem, InventoryItem, Item};
use schema::{self, item::dsl};
use types::UuidWrapper;

/// A repository for items.
#[derive(Clone)]
pub struct ItemRepository {
  context: DataContextInner,
}

impl ItemRepository {
  /// Creates a new item repository instance.
  pub fn new(context: &DataContext) -> Self {
    ItemRepository {
      context: context.clone(),
    }
  }

  /// Returns an item by its ID.
  pub fn find_by_id<I: Into<UuidWrapper>>(&self, id: I) -> Result<Option<Item>> {
    dsl::item
      .find(&id.into())
      .first::<Item>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Returns a character's equipment items.
  pub fn find_equipment_by_character_id(
    &self,
    character_id: i32,
  ) -> Result<Vec<(EquipmentItem, Item)>> {
    schema::equipment_item::table
      .inner_join(schema::item::table)
      .filter(schema::equipment_item::dsl::character_id.eq(&character_id))
      .load(&*self.context.access())
      .map_err(Into::into)
  }

  /// Returns an inventory's items.
  pub fn find_items_by_inventory_id(
    &self,
    inventory_id: i32,
  ) -> Result<Vec<(InventoryItem, Item)>> {
    schema::inventory_item::table
      .inner_join(schema::item::table)
      .filter(schema::inventory_item::dsl::inventory_id.eq(&inventory_id))
      .load(&*self.context.access())
      .map_err(Into::into)
  }

  /// Saves an item by updating or replacing it.
  pub fn save(&self, item: &Item) -> Result<()> {
    diesel::replace_into(dsl::item)
      .values(item)
      .execute(&*self.context.access())?;
    Ok(())
  }
}
