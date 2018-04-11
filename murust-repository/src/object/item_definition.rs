use schema::{item_definition, item_eligible_class};

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[primary_key(code)]
#[table_name = "item_definition"]
pub struct ItemDefinition {
  pub code: i32,
  pub name: String,
  pub equippable_slot: Option<i32>,
  pub max_durability: i32,
  pub width: i32,
  pub height: i32,
  pub drop_from_monster: bool,
  pub drop_level: i32,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "item_eligible_class"]
#[primary_key(item_code, class)]
pub struct ItemEligibleClass {
  pub item_code: i32,
  pub class: String,
}
