use schema::equipment_item;
use types::Id;

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(character_id, slot)]
#[table_name = "equipment_item"]
pub struct EquipmentItem {
  pub character_id: i32,
  pub item_id: Id,
  pub slot: i32,
}
