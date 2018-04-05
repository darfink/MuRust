#[macro_use]
extern crate diesel;
extern crate bcrypt;

// TODO: Error handling

pub use self::account::AccountInterface;
use diesel::Connection;
use diesel::sqlite::SqliteConnection;

mod account;
pub mod models;
mod schema;

pub struct Database {
  connection: SqliteConnection,
}

impl Database {
  pub fn new(filename: &str) -> Self {
    let connection = SqliteConnection::establish(&filename).unwrap();
    Database { connection }
  }
}

// TODO: Remove this and implement proper threading support
unsafe impl Sync for Database {}
