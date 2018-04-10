use schema::character;

#[derive(Identifiable, Queryable, AsChangeset)]
#[table_name = "character"]
pub struct Character {
  pub id: i32,
  pub slot: i32,
  pub name: String,
  pub level: i32,
  pub class: String,
  pub experience: i32,
  pub strength: i32,
  pub agility: i32,
  pub vitality: i32,
  pub energy: i32,
  pub command: i32,
  pub map: i32,
  pub position_x: i32,
  pub position_y: i32,
  pub player_kills: i32,
  pub inventory_id: i32,
  pub account_id: i32,
}
