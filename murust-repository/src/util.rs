use diesel;
use std::io;

/// Converts an Diesel error to an 'io::Error'
pub fn diesel_to_io(error: diesel::result::Error) -> io::Error {
  io::Error::new(io::ErrorKind::Other, error)
}
