use entities::item::{self, Item};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ItemStorage {
  width: u8,
  height: u8,
  items: HashMap<item::Id, Item>,
  grid: Vec<Option<item::Id>>,
}

impl ItemStorage {
  /// Creates a new storage with the specified size.
  pub fn new(width: u8, height: u8) -> Self {
    ItemStorage {
      width,
      height,
      items: HashMap::new(),
      grid: vec![None; (width * height) as usize],
    }
  }

  /// Adds an item at the first available slot, if any.
  ///
  /// ## Panics
  ///
  /// This panics if an item with the same ID has already been added.
  pub fn add_item(&mut self, mut item: Item) -> Result<(), Item> {
    for y in 0..=self.height.saturating_sub(item.definition.height) {
      for x in 0..=self.width.saturating_sub(item.definition.width) {
        let slot = y * self.width + x;
        match self.add_item_at_slot(slot, item) {
          Err(rebound) => item = rebound,
          Ok(()) => return Ok(()),
        }
      }
    }
    Err(item)
  }

  /// Adds an item at a specific slot.
  ///
  /// ## Panics
  ///
  /// This panics if an item with the same ID has already been added.
  pub fn add_item_at_slot(&mut self, slot: u8, item: Item) -> Result<(), Item> {
    if self.insert_item_id(slot, 0, &item) {
      assert!(self.items.insert(item.id, item).is_none());
      Ok(())
    } else {
      Err(item)
    }
  }

  /// Removes an item based on its ID.
  pub fn remove_item(&mut self, item_id: item::Id) -> Option<Item> {
    let item = self.items.remove(&item_id)?;
    let slot = self
      .get_item_slot(&item)
      .expect("retrieving map item from grid");
    self.clear_item_id(slot, &item);
    Some(item)
  }

  /// Removes any item that resides within the specified slot.
  pub fn remove_item_at_slot(&mut self, slot: u8) -> Option<Item> {
    self
      .get_item_id_from_slot(slot)
      .and_then(|id| self.remove_item(id))
  }

  /// Returns an item based on its ID.
  pub fn get_item(&mut self, item_id: item::Id) -> Option<&Item> { self.items.get(&item_id) }

  /// Returns any item that resides within the specified slot.
  pub fn get_item_at_slot(&self, slot: u8) -> Option<&Item> {
    self
      .get_item_id_from_slot(slot)
      .map(|id| self.items.get(&id).expect("retrieving grid item from map"))
  }

  /// Returns the most top-left slot of an item.
  pub fn get_item_slot(&self, item: &Item) -> Option<u8> {
    self
      .grid
      .iter()
      .position(|id| *id == Some(item.id))
      .map(|slot| slot as u8)
  }

  /// Returns an iterator of all items in the storage.
  pub fn items(&self) -> impl Iterator<Item = &Item> { self.items.iter().map(|(_, item)| item) }

  /// Returns total number of slots in the storage.
  pub fn slots(&self) -> usize { (self.width * self.height) as usize }

  /// Returns the number of free slots in the storage.
  pub fn slots_free(&self) -> usize { self.grid.iter().filter(|entry| entry.is_none()).count() }

  /// Clears the storage, removing any items within.
  pub fn clear(&mut self) {
    self.items.clear();
    for entry in &mut self.grid {
      *entry = None;
    }
  }

  /// Returns an item's ID from a slot.
  fn get_item_id_from_slot(&self, slot: u8) -> Option<item::Id> {
    self.grid.get(slot as usize).and_then(|entry| entry.clone())
  }

  /// Recursively inserts an item ID in the grid, based on backtracking.
  fn insert_item_id(&mut self, base_slot: u8, index: u8, item: &Item) -> bool {
    if index == item.definition.width * item.definition.height {
      return true;
    }

    let x = index % item.definition.width;
    let y = index / item.definition.width;
    let base_x = base_slot % self.width;
    let base_y = base_slot / self.width;

    if base_x + x >= self.width || base_y + y >= self.height {
      return false;
    }

    let slot = (base_slot + y * self.width + x) as usize;
    if *self.grid[slot].get_or_insert(item.id) == item.id {
      if self.insert_item_id(base_slot, index + 1, item) {
        return true;
      } else {
        self.grid[slot] = None;
      }
    }

    false
  }

  /// Clears an item ID from the grid.
  fn clear_item_id(&mut self, top_left_slot: u8, item: &Item) {
    for y in top_left_slot..(top_left_slot + item.definition.height) {
      for x in top_left_slot..(top_left_slot + item.definition.width) {
        self.grid[(y * self.width + x) as usize] = None;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use entities::{Item, ItemDefinition};
  use types::{ItemCode, ItemGroup, ItemStorage};

  #[test]
  fn add_item_1x1_top_left() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(1, 1);
    storage.add_item_at_slot(0, item).unwrap();

    assert_eq!(storage.slots_free(), 63);
    assert!(storage.get_item_at_slot(0).is_some());
  }

  #[test]
  fn add_item_2x2_bottom_right() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(2, 2);
    storage.add_item_at_slot(6 * 8 + 6, item).unwrap();
    assert_eq!(storage.slots_free(), 60);
  }

  #[test]
  fn add_item_fails_with_same_slot() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(1, 1);
    storage.add_item_at_slot(12, item).unwrap();

    let item = item_with_size(1, 1);
    assert!(storage.add_item_at_slot(12, item).is_err());
    assert_eq!(storage.slots_free(), 63);
  }

  #[test]
  fn add_item_fails_when_overlapping() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(2, 2);
    storage.add_item_at_slot(8, item).unwrap();

    let item = item_with_size(2, 2);
    assert!(storage.add_item_at_slot(8 * 2, item).is_err());
    assert_eq!(storage.slots_free(), 60);
  }

  #[test]
  fn add_item_fails_when_partially_outside_storage_horizontally() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(2, 1);
    assert!(storage.add_item_at_slot(7, item).is_err());
    assert!(storage.get_item_at_slot(7).is_none())
  }

  #[test]
  fn add_item_fails_when_partially_outside_storage_vertically() {
    let mut storage = ItemStorage::new(8, 8);

    let item = item_with_size(1, 2);
    assert!(storage.add_item_at_slot(7 * 8, item).is_err());
    assert!(storage.get_item_at_slot(7 * 8).is_none())
  }

  #[test]
  fn add_item_finds_slots_automatically() {
    let mut storage = ItemStorage::new(8, 8);
    assert!(storage.add_item_at_slot(9, item_with_size(6, 2)).is_ok());
    assert!(storage.add_item_at_slot(33, item_with_size(4, 3)).is_ok());
    assert!(storage.add_item_at_slot(37, item_with_size(2, 1)).is_ok());
    assert!(storage.add_item(item_with_size(3, 3)).is_ok());
    assert!(storage.get_item_at_slot(45).is_some());
    assert!(storage.add_item(item_with_size(5, 1)).is_ok());
    assert!(storage.add_item(item_with_size(8, 2)).is_err());
  }

  fn item_with_size(width: u8, height: u8) -> Item {
    let mut definition = ItemDefinition::new(ItemCode::new(ItemGroup::Helper, 0), "Test");
    definition.width = width;
    definition.height = height;
    Item::with_definition(definition)
  }
}
