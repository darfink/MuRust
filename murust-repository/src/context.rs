use diesel::{self, Connection, RunQueryDsl, sqlite::SqliteConnection};
use error::Result;
use schema;
use std::sync::{Arc, Mutex, MutexGuard};

/// The inner connection of a data context
#[derive(Clone)]
pub(crate) struct DataContextInner(Arc<Mutex<SqliteConnection>>);

impl DataContextInner {
  pub fn access(&self) -> MutexGuard<SqliteConnection> {
    self.0.lock().expect("accessing data context connection")
  }
}

/// A data storage context.
#[derive(Clone)]
pub struct DataContext(DataContextInner);

impl DataContext {
  /// Constructs a new data context.
  pub fn new(filename: &str) -> Result<Self> {
    let conn = SqliteConnection::establish(&filename)?;
    conn.execute("PRAGMA foreign_keys = ON")?;
    Ok(DataContext(DataContextInner(Arc::new(Mutex::new(conn)))))
  }

  /// Creates the default data schema (if table do not exist).
  pub fn initialize_schema(&self) -> Result<()> { self.execute_all(schema::DEFAULT) }

  /// Inserts the default test data.
  pub fn initialize_data(&self) -> Result<()> { self.execute_all(schema::TEST_DATA) }

  /// Returns the inner data context.
  pub(crate) fn inner(&self) -> DataContextInner { self.0.clone() }

  /// Executes all statements in an SQL string.
  fn execute_all<S: Into<String>>(&self, statements: S) -> Result<()> {
    let connection = self.0.access();
    statements
      .into()
      .split(";")
      .filter(|s| !s.is_whitespace())
      .map(|query| {
        diesel::sql_query(query)
          .execute(&*connection)
          .map_err(Into::into)
      })
      .collect::<Result<Vec<_>>>()
      .map(|_| ())
  }
}
