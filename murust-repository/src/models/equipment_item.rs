use schema::equipment_item;
use types::UuidWrapper;

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(character_id, slot)]
#[table_name = "equipment_item"]
pub struct EquipmentItem {
  pub character_id: i32,
  pub item_id: UuidWrapper,
  pub slot: i32,
}
