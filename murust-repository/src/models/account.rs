use schema::account;

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "account"]
pub struct Account {
  pub id: i32,
  pub username: String,
  pub password_hash: String,
  pub security_code: i32,
  pub email: String,
  pub logged_in: bool,
  pub failed_login_attempts: i32,
  pub failed_login_time: Option<i64>,
}
