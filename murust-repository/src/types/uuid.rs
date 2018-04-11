use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use std::io;

// TODO: Should this be named Uuid and have a uuid dependency?
#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Hash)]
#[sql_type = "Binary"]
pub struct Id([u8; 16]);

impl Id {
  /// Constructs a new ID instance.
  pub fn new(id: [u8; 16]) -> Self { Id(id) }

  /// Constructs a new ID instance from a hex string.
  pub fn from_hex(mut input: &str) -> Self {
    // TODO: Use proper error handling
    let mut data = [0u8; 16];
    for index in data.iter_mut() {
      let (hex, rest) = input.split_at(2);
      *index = u8::from_str_radix(hex, 16).unwrap();
      input = rest;
    }
    Id(data)
  }
}

impl<DB: Backend> ToSql<Binary, DB> for Id
where
  [u8]: ToSql<Binary, DB>,
{
  fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
    self.0.to_sql(out)
  }
}

impl<DB: Backend> FromSql<Binary, DB> for Id
where
  *const [u8]: FromSql<Binary, DB>,
{
  fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
    let data = <*const [u8] as FromSql<Binary, DB>>::from_sql(bytes)?;
    let slice = unsafe { data.as_ref().unwrap() };

    let mut id = Id([0u8; 16]);
    id.0.copy_from_slice(&slice);

    Ok(id)
  }
}
