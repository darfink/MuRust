use schema::account;

#[derive(Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
  pub username: &'a str,
  pub password: &'a str,
  pub email: &'a str,
  pub secret: i32,
}

#[derive(Queryable)]
pub struct Account {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub secret: i32,
}
