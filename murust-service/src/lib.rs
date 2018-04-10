#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate matches;

extern crate bcrypt;
extern crate murust_data_model;
extern crate murust_repository;

pub use self::account::{AccountLoginError, AccountService};
mod account;

#[cfg(test)]
mod tests {
  use super::*;
  use murust_repository::{AccountRepository, DataContext};
  use tempdir::TempDir;

  // TODO: Share this between crates somehow?
  fn setup_test_db() -> (TempDir, DataContext) {
    let tmp = TempDir::new("murust-repository").expect("creating tempdir");
    let path_buf = tmp.path().join("database.sqlite");
    let path = path_buf.to_str().expect("converting temp DB path");

    let database = DataContext::new(path).expect("creating DB");
    database
      .initialize_schema()
      .expect("creating default schema");
    database.initialize_data().expect("creating test data");

    (tmp, database)
  }

  #[test]
  fn account_login_and_logout() {
    let (_temp, db) = setup_test_db();
    let service = AccountService::new(AccountRepository::new(&db));
    let account = service.login("foobar", "test").unwrap().unwrap();
    assert!(service.logout(&account).is_ok());
  }

  #[test]
  fn account_lockout() {
    let (_temp, db) = setup_test_db();
    let service = AccountService::new(AccountRepository::new(&db));

    let fail = || service.login("foobar", "tist").unwrap();
    assert!(matches!(fail(), Err(AccountLoginError::InvalidPassword(_))));
    assert!(matches!(fail(), Err(AccountLoginError::TooManyAttempts(_))));
  }
}
