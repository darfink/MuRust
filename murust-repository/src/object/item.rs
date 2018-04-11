use schema::item;
use types::Id;

#[derive(Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name = "item"]
pub struct Item {
  pub id: Id,
  pub level: i32,
  pub durability: i32,
  pub item_definition_id: i32,
}
