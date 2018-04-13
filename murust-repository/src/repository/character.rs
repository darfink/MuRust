use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use models::Character;
use schema::character::dsl;
use std::io;
use util::diesel_to_io;

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
  pub fn find_by_id(&self, id: i32) -> io::Result<Option<Character>> {
    dsl::character
      .find(id)
      .first::<Character>(&*self.context.access())
      .optional()
      .map_err(diesel_to_io)
  }

  /// Returns a character by its name.
  pub fn find_by_name(&self, name: &str) -> io::Result<Option<Character>> {
    dsl::character
      .filter(dsl::name.eq(name))
      .first::<Character>(&*self.context.access())
      .optional()
      .map_err(diesel_to_io)
  }

  /// Returns an accounts characters.
  pub fn find_by_account_id(&self, account_id: i32) -> io::Result<Vec<Character>> {
    dsl::character
      .filter(dsl::account_id.eq(&account_id))
      .get_results::<Character>(&*self.context.access())
      .map_err(diesel_to_io)
  }
}
