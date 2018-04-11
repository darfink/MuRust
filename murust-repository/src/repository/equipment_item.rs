use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use object::EquipmentItem;
use schema::equipment_item::dsl;
use std::io;
use util::diesel_to_io;

/// A repository for equipment items.
#[derive(Clone)]
pub struct EquipmentItemRepository {
  context: DataContextInner,
}

impl EquipmentItemRepository {
  /// Creates a new equipment item repository instance.
  pub fn new(context: &DataContext) -> Self {
    EquipmentItemRepository {
      context: context.clone(),
    }
  }

  /// Returns a character's equipment items.
  pub fn find_by_character_id(&self, character_id: i32) -> io::Result<Vec<EquipmentItem>> {
    dsl::equipment_item
      .filter(dsl::character_id.eq(&character_id))
      .get_results::<EquipmentItem>(&*self.context.access())
      .map_err(diesel_to_io)
  }
}
