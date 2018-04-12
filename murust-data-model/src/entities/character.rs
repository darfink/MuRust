use configuration::{Class, Position, Equipment};
use entities::Inventory;

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
  pub position: Position,
  pub player_kills: i32,
  pub equipment: Equipment,
  pub inventory: Inventory,
}
