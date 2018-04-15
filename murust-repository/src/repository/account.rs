use boolinator::Boolinator;
use context::{DataContext, DataContextInner};
use diesel::{self, prelude::*};
use error::Result;
use models::Account;
use schema::account::dsl;

/// A repository for accounts.
#[derive(Clone)]
pub struct AccountRepository {
  context: DataContextInner,
}

impl AccountRepository {
  /// Creates a new account repository instance.
  pub fn new(context: &DataContext) -> Self {
    AccountRepository {
      context: context.inner(),
    }
  }

  /// Returns an account by its username.
  pub fn find_by_username(&self, username: &str) -> Result<Option<Account>> {
    dsl::account
      .filter(dsl::username.eq(username))
      .first::<Account>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Returns an account by its ID.
  pub fn find_by_id(&self, account_id: i32) -> Result<Option<Account>> {
    dsl::account
      .find(&account_id)
      .first::<Account>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }

  /// Creates a new account and returns it.
  pub fn create(
    &self,
    username: &str,
    password_hash: &str,
    security_code: i32,
    email: &str,
  ) -> Result<Account> {
    let context = self.context.access();
    diesel::insert_into(dsl::account)
      .values((
        dsl::username.eq(username),
        dsl::password_hash.eq(password_hash),
        dsl::security_code.eq(security_code),
        dsl::email.eq(email),
      ))
      .execute(&*context)
      .and_then(|_| dsl::account.order(dsl::id.desc()).first(&*context))
      .map_err(Into::into)
  }

  /// Saves modifications to an account.
  pub fn update(&self, account: &Account) -> Result<()> {
    diesel::update(dsl::account)
      .set(account)
      .execute(&*self.context.access())?;
    Ok(())
  }

  /// Deletes an account by its ID.
  pub fn delete(&self, account_id: &i32) -> Result<()> {
    diesel::delete(dsl::account.filter(dsl::id.eq(account_id)))
      .execute(&*self.context.access())
      .and_then(|count| (count == 1).ok_or(diesel::result::Error::NotFound))
      .map_err(Into::into)
  }
}
