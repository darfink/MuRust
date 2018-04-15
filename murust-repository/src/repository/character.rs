use boolinator::Boolinator;
use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use error::Result;
use models::Character;
use schema::character::dsl;
use types::UuidWrapper;

/// A repository for characters.
#[derive(Clone)]
pub struct CharacterRepository {
  context: DataContextInner,
}

impl CharacterRepository {
  /// Creates a new character repository instance.
  pub fn new(context: &DataContext) -> Self {
    CharacterRepository {
      context: context.inner(),
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

  /// Creates a new character and returns it.
  pub fn create<I: Into<UuidWrapper>>(
    &self,
    slot: i32,
    name: &str,
    class: &str,
    map: i32,
    position_x: i32,
    position_y: i32,
    inventory_id: I,
    account_id: i32,
  ) -> Result<Character> {
    let context = self.context.access();
    diesel::insert_into(dsl::character)
      .values((
        dsl::slot.eq(slot),
        dsl::name.eq(name),
        dsl::class.eq(class),
        dsl::map.eq(map),
        dsl::position_x.eq(position_x),
        dsl::position_y.eq(position_y),
        dsl::inventory_id.eq(&inventory_id.into()),
        dsl::account_id.eq(account_id),
      ))
      .execute(&*context)
      .and_then(|_| dsl::character.order(dsl::id.desc()).first(&*context))
      .map_err(Into::into)
  }

  /// Deletes a character by its ID.
  pub fn delete(&self, character_id: &i32) -> Result<()> {
    diesel::delete(dsl::character.filter(dsl::id.eq(character_id)))
      .execute(&*self.context.access())
      .and_then(|count| (count == 1).ok_or(diesel::result::Error::NotFound))
      .map_err(Into::into)
  }
}
