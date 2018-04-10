use configuration::ItemStorage;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Inventory {
  pub id: i32,
  pub storage: ItemStorage,
  pub money: u32,
}

impl Deref for Inventory {
  type Target = ItemStorage;

  fn deref(&self) -> &Self::Target { &self.storage }
}

impl DerefMut for Inventory {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
}
