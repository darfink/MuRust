use diesel::{self, Connection, RunQueryDsl, sqlite::SqliteConnection};
use schema;
use std::{io, sync::{Arc, Mutex, MutexGuard}};
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

  /// Creates the default data schema (if table do not exist).
  pub fn initialize_schema(&self) -> io::Result<()> { self.execute_all(schema::DEFAULT) }

  /// Inserts the default test data.
  pub fn initialize_data(&self) -> io::Result<()> { self.execute_all(schema::TEST_DATA) }

  /// Returns the inner data context.
  pub(crate) fn clone(&self) -> DataContextInner { self.0.clone() }

  /// Executes all statements in an SQL string.
  fn execute_all<S: Into<String>>(&self, statements: S) -> io::Result<()> {
    let connection = self.0.access();
    statements
      .into()
      .split(";")
      .filter(|s| !s.is_whitespace())
      .map(|query| diesel::sql_query(query).execute(&*connection))
      .collect::<Result<Vec<_>, _>>()
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
      .map(|_| ())
  }
}
