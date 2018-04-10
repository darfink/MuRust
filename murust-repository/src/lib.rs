#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate boolinator;

pub use self::context::DataContext;
pub use self::repository::AccountRepository;

mod context;
pub mod object;
mod repository;
mod schema;
mod util;

#[cfg(test)]
mod tests {
  use DataContext;

  lazy_static! {
    static ref DATABASE: DataContext = { DataContext::new("database.sqlite").unwrap() };
  }

  #[test]
  fn account_find() {
    use repository::AccountRepository;
    let accounts = AccountRepository::new(&*DATABASE);

    assert!(accounts.find_by_username("foobar").unwrap().is_some());
    assert!(accounts.find_by_id(&1).unwrap().is_some());
  }

  #[test]
  fn account_add_remove() {
    use repository::AccountRepository;
    let accounts = AccountRepository::new(&*DATABASE);

    let account = accounts
      .create(
        "fajbar",
        "$2y$07$zFM0q8YmKjaYW4Hig6AFz.wroa/eG5DSK4ST9Y0KS4hDw5Jepw31a",
        123456,
        "fajbar@mail.com",
      )
      .unwrap();
    assert!(accounts.delete(&account.id).is_ok());
  }
}
