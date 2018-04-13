use diesel;

/// The repository error type.
#[derive(Debug, Fail)]
#[fail(display = "An error occurred.")]
pub enum Error {
  #[fail(display = "A connection error occurred.")]
  Connection(#[cause] diesel::ConnectionError),
  #[fail(display = "An query error occurred.")]
  Query(#[cause] diesel::result::Error),
}

impl From<diesel::ConnectionError> for Error {
  fn from(error: diesel::ConnectionError) -> Self { Error::Connection(error) }
}

impl From<diesel::result::Error> for Error {
  fn from(error: diesel::result::Error) -> Self { Error::Query(error) }
}

/// The default result type.
pub type Result<T> = ::std::result::Result<T, Error>;
