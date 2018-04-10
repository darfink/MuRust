use configuration::Storage;

#[derive(Debug)]
pub struct Inventory {
  pub id: i32,
  pub storage: Storage,
  pub money: u32,
}
