use entities::Item;
use num_traits::FromPrimitive;
use std::ops::{Index, IndexMut};
use types::ItemSlot;

#[derive(Default, Debug)]
pub struct Equipment([Option<Item>; ItemSlot::SIZE]);

impl Index<ItemSlot> for Equipment {
  type Output = Option<Item>;

  /// Returns the item in the equipment slot.
  fn index(&self, index: ItemSlot) -> &Self::Output { &self.0[index as usize] }
}

impl IndexMut<ItemSlot> for Equipment {
  /// Returns a mutable item in the equipment slot.
  fn index_mut(&mut self, index: ItemSlot) -> &mut Self::Output { &mut self.0[index as usize] }
}

impl<'a> IntoIterator for &'a Equipment {
  type Item = (ItemSlot, &'a Option<Item>);
  type IntoIter = EquipmentIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    EquipmentIterator {
      equipment: self,
      index: 0,
    }
  }
}

/// An iterator over a character's equipment.
pub struct EquipmentIterator<'a> {
  equipment: &'a Equipment,
  index: usize,
}

impl<'a> Iterator for EquipmentIterator<'a> {
  type Item = (ItemSlot, &'a Option<Item>);

  fn next(&mut self) -> Option<Self::Item> {
    self.equipment.0.get(self.index).map(|item| {
      let slot = ItemSlot::from_usize(self.index).unwrap();
      self.index += 1;
      (slot, item)
    })
  }
}
