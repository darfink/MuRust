use schema::{inventory, inventory_item};
use types::Id;

#[derive(Identifiable, Queryable, AsChangeset)]
#[table_name = "inventory"]
pub struct Inventory {
  pub id: i32,
  pub width: i32,
  pub height: i32,
  pub money: i32,
}

#[derive(Identifiable, Queryable, AsChangeset)]
#[primary_key(inventory_id, slot)]
#[table_name = "inventory_item"]
pub struct InventoryItem {
  pub inventory_id: i32,
  pub item_id: Id,
  pub slot: i32,
}
