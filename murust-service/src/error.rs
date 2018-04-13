use bcrypt::BcryptError;
use mapping::MappingError;
use murust_repository;
use std::time::SystemTimeError;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "The associated model entry '{}' was not found", _0)]
  MissingAssociation(String),
  #[fail(display = "The specified entity does not exist in the persistence storage")]
  MissingPersistence,
  #[fail(display = "A repository error occurred.")]
  Repository(#[cause] murust_repository::Error),
  #[fail(display = "An entity mapping error occurred.")]
  Mapping(#[cause] MappingError),
  #[fail(display = "An hasing error occurred.")]
  Hashing(#[cause] BcryptError),
  #[fail(display = "An system time error occurred.")]
  SystemTime(#[cause] SystemTimeError),
}

impl From<murust_repository::Error> for Error {
  fn from(error: murust_repository::Error) -> Self { Error::Repository(error) }
}

impl From<MappingError> for Error {
  fn from(error: MappingError) -> Self { Error::Mapping(error) }
}

impl From<BcryptError> for Error {
  fn from(error: BcryptError) -> Self { Error::Hashing(error) }
}

impl From<SystemTimeError> for Error {
  fn from(error: SystemTimeError) -> Self { Error::SystemTime(error) }
}

/// The default result type.
pub type Result<T> = ::std::result::Result<T, Error>;
