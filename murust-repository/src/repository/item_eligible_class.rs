use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use object::ItemEligibleClass;
use schema::item_eligible_class::dsl;
use std::io;
use util::diesel_to_io;

/// A repository for eligible item classes.
#[derive(Clone)]
pub struct ItemEligibleClassRepository {
  context: DataContextInner,
}

impl ItemEligibleClassRepository {
  /// Creates a new eligible item class repository instance.
  pub fn new(context: &DataContext) -> Self {
    ItemEligibleClassRepository {
      context: context.clone(),
    }
  }

  /// Returns an item definitions eligible clases.
  pub fn find_by_item_definition_code(&self, item_definition_code: i32) -> io::Result<Vec<ItemEligibleClass>> {
    dsl::item_eligible_class
      .filter(dsl::item_definition_code.eq(&item_definition_code))
      .get_results::<ItemEligibleClass>(&*self.context.access())
      .map_err(diesel_to_io)
  }
}
