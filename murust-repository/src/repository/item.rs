use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use object::Item;
use schema::item::dsl;
use std::io;
use types::Id;
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

  /// Saves an item by updating or replacing it.
  pub fn save(&self, item: &Item) -> io::Result<()> {
    diesel::replace_into(dsl::item)
      .values(item)
      .execute(&*self.context.access())
      .map_err(diesel_to_io)?;
    Ok(())
  }
}
