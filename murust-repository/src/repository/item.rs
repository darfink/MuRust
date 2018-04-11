use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use object::Item;
use schema::item::dsl;
use types::Id;
use std::io;
use util::diesel_to_io;

/// A repository for items.
#[derive(Clone)]
pub struct ItemRepository {
  context: DataContextInner,
}

impl ItemRepository {
  /// Creates a new item repository instance.
  pub fn new(context: &DataContext) -> Self {
    ItemRepository {
      context: context.clone(),
    }
  }

  /// Returns an item by its ID.
  pub fn find_by_id(&self, id: &Id) -> io::Result<Option<Item>> {
    dsl::item
      .find(id)
      .first::<Item>(&*self.context.access())
      .optional()
      .map_err(diesel_to_io)
  }
}
