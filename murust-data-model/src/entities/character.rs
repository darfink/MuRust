use configuration::{Class, ItemSlot};
use entities::{Inventory, Item};
use enum_map::EnumMap;

#[derive(Debug)]
pub struct Character {
  pub id: i32,
  pub slot: u8,
  pub name: String,
  pub level: u16,
  pub class: Class,
  pub experience: u32,
  pub strength: u16,
  pub agility: u16,
  pub vitality: u16,
  pub energy: u16,
  pub command: u16,
  pub map: u8,
  pub position_x: u8,
  pub position_y: u8,
  pub player_kills: i32,
  pub equipment: Equipment,
  pub inventory: Inventory,
}

// TODO: Where should this be?
pub type Equipment = EnumMap<ItemSlot, Option<Item>>;
