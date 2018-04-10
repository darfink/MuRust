#[cfg(test)]
extern crate tempdir;

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
  use tempdir::TempDir;

  // TODO: Share this between crates somehow?
  fn setup_test_db() -> (TempDir, DataContext) {
    let tmp = TempDir::new("murust-repository").expect("creating tempdir");
    let path_buf = tmp.path().join("database.sqlite");
    let path = path_buf.to_str().expect("converting temp DB path");

    let database = DataContext::new(path).expect("creating DB");
    database.initialize_schema().expect("creating default schema");
    database.initialize_data().expect("creating test data");

    (tmp, database)
  }

  #[test]
  fn account_find() {
    use repository::AccountRepository;

    let (_temp, db) = setup_test_db();
    let accounts = AccountRepository::new(&db);

    assert!(accounts.find_by_username("foobar").unwrap().is_some());
    assert!(accounts.find_by_id(&1).unwrap().is_some());
  }

  #[test]
  fn account_add_remove() {
    use repository::AccountRepository;

    let (_temp, db) = setup_test_db();
    let accounts = AccountRepository::new(&db);

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
