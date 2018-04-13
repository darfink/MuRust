use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use error::Result;
use models::Character;
use schema::character::dsl;

/// A repository for characters.
#[derive(Clone)]
pub struct CharacterRepository {
  context: DataContextInner,
}

impl CharacterRepository {
  /// Creates a new character repository instance.
  pub fn new(context: &DataContext) -> Self {
    CharacterRepository {
      context: context.clone(),
    }
  }

  /// Returns a character by its ID.
  pub fn find_by_id(&self, id: i32) -> Result<Option<Character>> {
    dsl::character
      .find(id)
      .first::<Character>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Returns a character by its name.
  pub fn find_by_name(&self, name: &str) -> Result<Option<Character>> {
    dsl::character
      .filter(dsl::name.eq(name))
      .first::<Character>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Returns an accounts characters.
  pub fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Character>> {
    dsl::character
      .filter(dsl::account_id.eq(&account_id))
      .get_results::<Character>(&*self.context.access())
      .map_err(Into::into)
  }
}
