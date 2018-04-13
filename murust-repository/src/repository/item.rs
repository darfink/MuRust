use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use models::{EquipmentItem, InventoryItem, Item};
use schema::{self, item::dsl};
use std::io;
use types::UuidWrapper;
use util::diesel_to_io;

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
  pub fn find_by_id<I: Into<UuidWrapper>>(&self, id: I) -> io::Result<Option<Item>> {
    dsl::item
      .find(&id.into())
      .first::<Item>(&*self.context.access())
      .optional()
      .map_err(diesel_to_io)
  }

  /// Returns a character's equipment items.
  pub fn find_equipment_by_character_id(
    &self,
    character_id: i32,
  ) -> io::Result<Vec<(EquipmentItem, Item)>> {
    schema::equipment_item::table
      .inner_join(schema::item::table)
      .filter(schema::equipment_item::dsl::character_id.eq(&character_id))
      .load(&*self.context.access())
      .map_err(diesel_to_io)
  }

  /// Returns an inventory's items.
  pub fn find_items_by_inventory_id(
    &self,
    inventory_id: i32,
  ) -> io::Result<Vec<(InventoryItem, Item)>> {
    schema::inventory_item::table
      .inner_join(schema::item::table)
      .filter(schema::inventory_item::dsl::inventory_id.eq(&inventory_id))
      .load(&*self.context.access())
      .map_err(diesel_to_io)
  }

  /// Saves an item by updating or replacing it.
  pub fn save(&self, item: &Item) -> io::Result<()> {
    diesel::replace_into(dsl::item)
      .values(item)
      .execute(&*self.context.access())
      .map_err(diesel_to_io)?;
    Ok(())
  }
}
