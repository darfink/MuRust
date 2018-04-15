use std::ops::{Deref, DerefMut};
use types::ItemStorage;
use uuid::Uuid;

/// The type of ID used by inventory entities.
pub type Id = Uuid;

#[derive(Debug)]
pub struct Inventory {
  pub id: Id,
  pub storage: ItemStorage,
  pub money: u32,
}

impl Inventory {
  /// Constructs a new inventory instance.
  pub fn new(width: u8, height: u8) -> Self {
    Inventory {
      id: Id::new_v4(),
      storage: ItemStorage::new(width, height),
      money: 0,
    }
  }
}

impl Deref for Inventory {
  type Target = ItemStorage;

  fn deref(&self) -> &Self::Target { &self.storage }
}

impl DerefMut for Inventory {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
}
