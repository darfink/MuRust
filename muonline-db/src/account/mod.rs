use models::Account;
use {bcrypt, Database};

pub trait AccountInterface {
  fn authenticate(&self, iusername: &str, ipassword: &str) -> Option<Account>;
}

impl AccountInterface for Database {
  fn authenticate(&self, iusername: &str, ipassword: &str) -> Option<Account> {
    use diesel::prelude::*;
    use schema::account::dsl::*;

    account
      .filter(username.eq(iusername))
      .first::<Account>(&self.connection)
      .optional()
      .unwrap()
      .into_iter()
      .filter(|table| bcrypt::verify(ipassword, &table.password).unwrap())
      .next()
  }
}

#[cfg(test)]
mod tests {
  use super::AccountInterface;
  use {bcrypt, diesel, schema, Database};

  fn hash_password(input: &str) -> String {
    const BCRYPT_COST: u32 = 7;
    bcrypt::hash(input, BCRYPT_COST).unwrap()
  }

  #[test]
  fn account_insert() {
    use diesel::prelude::*;
    use models::NewAccount;

    let password = hash_password("test");
    let account = NewAccount {
      username: "atomen",
      password: password.as_ref(),
      email: "test@mail.com",
      secret: 1234567,
    };

    let db = Database::new("database.sqlite");
    diesel::insert_into(schema::account::table)
      .values(&account)
      .execute(&db.connection)
      .expect("Error inserting account");
  }

  #[test]
  fn account_login() {
    let db = Database::new("database.sqlite");
    let account = db.login("atomen", "test");
    assert!(account.is_some());
  }
}
