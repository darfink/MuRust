use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use error::Result;
use models::ItemEligibleClass;
use schema::item_eligible_class::dsl;

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

  /// Returns an item definition's eligible classes.
  pub fn find_by_item_code(&self, item_code: i32) -> Result<Vec<ItemEligibleClass>> {
    dsl::item_eligible_class
      .filter(dsl::item_code.eq(&item_code))
      .get_results::<ItemEligibleClass>(&*self.context.access())
      .map_err(Into::into)
  }
}
