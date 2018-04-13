use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use std::{io, ops::Deref};
use uuid::Uuid;

/// An UUID wrapper since diesel does not support UUID for Sqlite.
#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Copy, Clone, Hash)]
#[sql_type = "Binary"]
pub struct UuidWrapper(Uuid);

impl From<Uuid> for UuidWrapper {
  fn from(uuid: Uuid) -> Self { UuidWrapper(uuid) }
}

impl Deref for UuidWrapper {
  type Target = Uuid;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl<DB: Backend> ToSql<Binary, DB> for UuidWrapper
where
  [u8]: ToSql<Binary, DB>,
{
  fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
    self.0.as_bytes().to_sql(out)
  }
}

impl<DB: Backend> FromSql<Binary, DB> for UuidWrapper
where
  *const [u8]: FromSql<Binary, DB>,
{
  fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
    let data = <*const [u8] as FromSql<Binary, DB>>::from_sql(bytes)?;
    let slice = unsafe { data.as_ref().unwrap() };

    let uuid = Uuid::from_bytes(slice)?;
    Ok(uuid.into())
  }
}
