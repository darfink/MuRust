#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate matches;

#[macro_use]
extern crate failure;
extern crate bcrypt;
extern crate murust_data_model;
extern crate murust_repository;
extern crate num_traits;
extern crate uuid;

pub use self::error::Error;
pub use self::manager::ServiceManager;
pub use self::services::*;

mod error;
mod manager;
mod mapping;
mod services;

#[cfg(test)]
mod tests {
  use super::*;
  use murust_data_model::entities::item;
  use murust_data_model::types::ItemSlot;
  use murust_repository::*;
  use tempdir::TempDir;

  fn setup_test_env() -> (TempDir, ServiceManager) {
    let tmp = TempDir::new("murust-repository").expect("creating tempdir");
    let path_buf = tmp.path().join("database.sqlite");
    let path = path_buf.to_str().expect("converting temp DB path");

    let database = DataContext::new(path).expect("creating DB");
    database
      .initialize_schema()
      .expect("creating default schema");
    database.initialize_data().expect("creating test data");

    (tmp, ServiceManager::new(database))
  }

  #[test]
  fn successful_account_login_and_logout() {
    let (_temp, manager) = setup_test_env();
    let service = manager.account_service();
    let account = service.login("foobar", "test").unwrap().unwrap();
    assert!(service.logout(&account).is_ok());
  }

  #[test]
  fn account_lockout_after_failed_attempts() {
    let (_temp, manager) = setup_test_env();
    let service = manager.account_service();

    let fail = || service.login("foobar", "tist").unwrap();
    assert!(matches!(fail(), Err(AccountLoginError::InvalidPassword(_))));
    assert!(matches!(fail(), Err(AccountLoginError::TooManyAttempts(_))));
  }

  #[test]
  fn find_character_by_account_id() {
    let (_temp, manager) = setup_test_env();
    let service = manager.character_service();
    let characters = service.find_by_account_id(1).unwrap();

    assert_eq!(characters.len(), 1);
    assert_eq!(characters[0].name, "deadbeef");

    let weapon = characters[0].equipment[ItemSlot::WeaponRight].as_ref();
    assert_eq!(weapon.unwrap().definition.name, "Short Sword");
  }

  #[test]
  fn find_items_by_id() {
    let (_temp, manager) = setup_test_env();
    let service = manager.item_service();

    let id = item::Id::parse_str("6606af63a93c11e4979700505690798f").unwrap();
    let item = service.find_by_id(&id).unwrap().unwrap();

    assert_eq!(item.level, 2);
    assert_eq!(item.definition.name, "Kris");
  }
}
