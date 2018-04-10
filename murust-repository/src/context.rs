use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};
use util::diesel_to_io;

/// The inner connection of a data context
#[derive(Clone)]
pub(crate) struct DataContextInner(Arc<Mutex<SqliteConnection>>);

impl DataContextInner {
  pub fn access(&self) -> MutexGuard<SqliteConnection> {
    self.0.lock().expect("accessing data context connection")
  }
}

/// A data storage context.
pub struct DataContext(DataContextInner);

impl DataContext {
  /// Constructs a new data context.
  pub fn new(filename: &str) -> io::Result<Self> {
    SqliteConnection::establish(&filename)
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
      .and_then(|conn| {
        conn
          .execute("PRAGMA foreign_keys = ON")
          .map(|_| conn)
          .map_err(diesel_to_io)
      })
      .map(|conn| DataContext(DataContextInner(Arc::new(Mutex::new(conn)))))
  }

  /// Returns the inner data context.
  pub(crate) fn clone(&self) -> DataContextInner { self.0.clone() }
}
