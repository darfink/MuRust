#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate lazy_static;

extern crate bcrypt;
extern crate murust_data_model;
extern crate murust_repository;

pub use self::account::{AccountLoginError, AccountService};
mod account;

#[cfg(test)]
mod tests {
  use super::*;
  use murust_repository::{AccountRepository, DataContext};

  lazy_static! {
    static ref DATABASE: DataContext = { DataContext::new("database.sqlite").unwrap() };
  }

  #[test]
  fn account_login_and_logout() {
    let service = AccountService::new(AccountRepository::new(&*DATABASE));
    let account = service.login("foobar", "test").unwrap().unwrap();
    assert!(service.logout(&account).is_ok());
  }

  /*#[test]
  fn account_lockout() {
    let service = AccountService::new(AccountRepository::new(&*DATABASE));
    let fail = || service.login("foobar", "tist").unwrap();

    assert!(matches!(fail(), Err(AccountLoginError::InvalidPassword(_))));
    assert!(matches!(fail(), Err(AccountLoginError::InvalidPassword(_))));
    assert!(matches!(fail(), Err(AccountLoginError::TooManyAttempts(_))));
  }*/
}
