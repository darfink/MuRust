use schema::equipment_item;

#[derive(Identifiable, Queryable, AsChangeset)]
#[primary_key(character_id, slot)]
#[table_name = "equipment_item"]
pub struct EquipmentItem {
  pub character_id: i32,
  pub item_id: i32,
  pub slot: i32,
}
