use schema::item;
use types::UuidWrapper;

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Debug)]
#[table_name = "item"]
pub struct Item {
  pub id: UuidWrapper,
  pub code: i32,
  pub level: i32,
  pub durability: i32,
}
