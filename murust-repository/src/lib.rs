#[cfg(test)]
extern crate tempdir;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate diesel;
extern crate boolinator;
extern crate uuid;

pub use self::context::DataContext;
pub use self::repository::*;
pub use error::Error;

mod context;
mod error;
pub mod models;
mod repository;
mod schema;
mod types;

#[cfg(test)]
mod tests {
  use super::*;
  use tempdir::TempDir;
  use uuid::Uuid;

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
  fn find_account_by_username_and_id() {
    let (_temp, db) = setup_test_db();
    let accounts = AccountRepository::new(&db);

    assert!(accounts.find_by_username("foobar").unwrap().is_some());
    assert!(accounts.find_by_id(&1).unwrap().is_some());
  }

  #[test]
  fn add_and_then_remove_account() {
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

  #[test]
  fn find_character_by_name() {
    let (_temp, db) = setup_test_db();
    let repository = CharacterRepository::new(&db);
    assert!(repository.find_by_name("deadbeef").unwrap().is_some());
  }

  #[test]
  fn find_characters_from_account() {
    let (_temp, db) = setup_test_db();
    let repository = CharacterRepository::new(&db);

    let characters = repository.find_by_account_id(1).unwrap();
    assert_eq!(characters.len(), 1);
    assert_eq!(characters[0].name, "deadbeef");
  }

  #[test]
  fn find_item_by_id_and_update() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    let id = Uuid::parse_str("6606af63a93c11e4979700505690798f").unwrap();
    let mut item = repository.find_by_id(id).unwrap().unwrap();

    assert_eq!(item.level, 2);
    assert_eq!(item.durability, 20);

    item.level = 3;
    repository.save(&item).unwrap();

    let item = repository.find_by_id(id).unwrap().unwrap();
    assert_eq!(item.level, 3);
  }

  #[test]
  fn find_item_definition_from_item_code() {
    let (_temp, db) = setup_test_db();
    let repository = ItemDefinitionRepository::new(&db);

    let definition = repository.find_by_item_code(2).unwrap().unwrap();
    assert_eq!(definition.height, 3);
    assert_eq!(definition.name, "Rapier");
  }

  #[test]
  fn find_inventory_by_id() {
    let (_temp, db) = setup_test_db();
    let repository = InventoryRepository::new(&db);

    let id = Uuid::parse_str("587d12b748364673a0989476894283e4").unwrap();
    let inventory = repository.find_by_id(id).unwrap().unwrap();
    assert_eq!(inventory.width, 8);
    assert_eq!(inventory.height, 8);
    assert_eq!(inventory.money, 1337);
  }

  #[test]
  fn find_inventory_items_from_inventory() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    let id = Uuid::parse_str("587d12b748364673a0989476894283e4").unwrap();
    let items = repository.find_inventory_contents_by_id(id).unwrap();
    assert_eq!(items.len(), 1);

    let id = Uuid::parse_str("6606af63a93c11e4979700505690798f").unwrap();
    assert_eq!(*items[0].0.item_id, id);
  }

  #[test]
  fn clear_items_from_inventory() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    let id = Uuid::parse_str("587d12b748364673a0989476894283e4").unwrap();
    repository.clear_inventory_by_id(id).unwrap();
    assert!(
      repository
        .find_inventory_contents_by_id(id)
        .unwrap()
        .is_empty()
    );
  }

  #[test]
  fn find_equipment_items_from_character() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    let items = repository.find_equipment_by_character_id(1).unwrap();
    assert_eq!(items.len(), 7);

    let id = Uuid::parse_str("3f06af63a93c11e4979700505690773f").unwrap();
    assert!(items.iter().any(|(i, _)| *i.item_id == id));
  }

  #[test]
  fn delete_equipment_items_from_character() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    repository.delete_equipment_by_character_id(1).unwrap();
    let items = repository.find_equipment_by_character_id(1).unwrap();
    assert!(items.is_empty());
  }

  #[test]
  fn find_item_eligible_classes_from_item_code() {
    let (_temp, db) = setup_test_db();
    let repository = ItemEligibleClassRepository::new(&db);

    let classes = repository.find_by_item_code(2).unwrap();
    assert_eq!(classes.len(), 4);
  }
}
