use schema::{inventory, inventory_item};
use types::UuidWrapper;

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[table_name = "inventory"]
pub struct Inventory {
  pub id: i32,
  pub width: i32,
  pub height: i32,
  pub money: i32,
}

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(inventory_id, slot)]
#[table_name = "inventory_item"]
pub struct InventoryItem {
  pub inventory_id: i32,
  pub item_id: UuidWrapper,
  pub slot: i32,
}
