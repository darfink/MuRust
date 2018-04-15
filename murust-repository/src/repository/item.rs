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
      context: context.inner(),
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
  pub fn find_inventory_contents_by_id<I: Into<UuidWrapper>>(
    &self,
    inventory_id: I,
  ) -> Result<Vec<(InventoryItem, Item)>> {
    schema::inventory_item::table
      .inner_join(schema::item::table)
      .filter(schema::inventory_item::dsl::inventory_id.eq(&inventory_id.into()))
      .load(&*self.context.access())
      .map_err(Into::into)
  }

  /// Deletes a character's equipment (including the items).
  pub fn delete_equipment_by_character_id(&self, character_id: i32) -> Result<()> {
    let conn = self.context.access();

    // Cascading delete's the equipment items automatically
    let item_ids = schema::equipment_item::table
      .select(schema::equipment_item::dsl::item_id)
      .filter(schema::equipment_item::dsl::character_id.eq(&character_id));
    diesel::delete(dsl::item.filter(dsl::id.eq_any(item_ids))).execute(&*conn)?;
    Ok(())
  }

  /// Deletes an inventory's contents (including the items).
  pub fn clear_inventory_by_id<I: Into<UuidWrapper>>(&self, inventory_id: I) -> Result<()> {
    let conn = self.context.access();
    let inventory_id = inventory_id.into();

    // Cascading delete's the inventory items automatically
    let item_ids = schema::inventory_item::table
      .select(schema::inventory_item::dsl::item_id)
      .filter(schema::inventory_item::dsl::inventory_id.eq(&inventory_id));
    diesel::delete(dsl::item.filter(dsl::id.eq_any(item_ids))).execute(&*conn)?;
    Ok(())
  }

  /// Saves an item by updating or replacing it.
  pub fn save(&self, item: &Item) -> Result<()> {
    diesel::replace_into(dsl::item)
      .values(item)
      .execute(&*self.context.access())?;
    Ok(())
  }
}
