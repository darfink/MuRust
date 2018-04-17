use failure::{Context, Error};
use std::fmt::Display;

/// A shorthand for generating a context error.
pub fn cxerr<D: Display + Send + Sync + 'static>(message: D) -> Error {
  Context::new(message).into()
}

/// The default result type used.
pub type Result<T> = ::std::result::Result<T, Error>;
