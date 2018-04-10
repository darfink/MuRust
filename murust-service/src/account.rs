use bcrypt;
use murust_data_model::entities::Account;
use murust_repository::{object, AccountRepository};
use std::io;

/// A collection of possible login errors.
#[derive(Debug)]
pub enum AccountLoginError {
  InvalidUsername,
  InvalidPassword(Account),
  AlreadyConnected(Account),
  TooManyAttempts(Account),
}

/// A service for account management.
#[derive(Clone)]
pub struct AccountService {
  /// The database connection.
  repository: AccountRepository,
  /// The cost used for the hashing algorithm.
  hashing_cost: u32,
  /// The number of attempts until a user will start being timed out
  lockout_attempts: u32,
  /// The maximum time a user can be locked out.
  lockout_time_max: u64,
}

impl AccountService {
  /// Constructs a new account service.
  pub fn new(repository: AccountRepository) -> Self {
    // TODO: These settings should be supplied by injection
    AccountService {
      repository,
      hashing_cost: 10,
      lockout_attempts: 1,
      lockout_time_max: 60 * 24 * 2,
    }
  }

  /// Returns an account if the provided credentials are correct.
  pub fn login(
    &self,
    username: &str,
    password: &str,
  ) -> io::Result<Result<Account, AccountLoginError>> {
    // Fetches an account that is not timed out, matching the supplied username
    let mut account = match self.repository.find_by_username(username)? {
      None => return Ok(Err(AccountLoginError::InvalidUsername)),
      Some(account) => account,
    };

    if self.is_timed_out(&account)? {
      Ok(Err(AccountLoginError::TooManyAttempts(
        util::map_to_entity(account)?,
      )))
    } else if !self.is_valid_password(password, &account)? {
      self.increment_login_attempts(&mut account)?;
      Ok(Err(AccountLoginError::InvalidPassword(
        util::map_to_entity(account)?,
      )))
    } else if account.logged_in {
      Ok(Err(AccountLoginError::AlreadyConnected(
        util::map_to_entity(account)?,
      )))
    } else {
      self.login_account(&mut account)?;
      util::map_to_entity(account).map(Ok)
    }
  }

  /// Logs out an account from the underlying repository.
  pub fn logout(&self, account: &Account) -> io::Result<()> {
    let mut object = self
      .repository
      .find_by_id(&account.id)?
      .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
    object.logged_in = true;
    self.repository.update(&object)
  }

  /// Creates a new account and returns it as an object.
  pub fn create(
    &self,
    username: &str,
    password: &str,
    security_code: u32,
    email: &str,
  ) -> io::Result<Account> {
    let password_hash = self.hash_password(password)?;
    self
      .repository
      .create(username, &password_hash, security_code as i32, email)
      .and_then(util::map_to_entity)
  }

  /// Updates an account's underlying storage.
  pub fn update(&self, account: &Account) -> io::Result<()> {
    let mut object = self
      .repository
      .find_by_id(&account.id)?
      .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    // TODO: Can these allocations be prevented?
    if account.username != object.username { object.username = account.username.clone(); }
    if account.email != object.email { object.email = account.email.clone(); }

    object.security_code = account.security_code as i32;
    self.repository.update(&object)
  }

  /// Removes an account from the underlying storage.
  pub fn delete(&self, account: Account) -> Result<(), (io::Error, Account)> {
    self
      .repository
      .delete(&account.id)
      .map_err(|error| (error, account))
  }

  /// Returns the hash of a password.
  fn hash_password(&self, password: &str) -> io::Result<String> {
    bcrypt::hash(password, self.hashing_cost).map_err(util::bcrypt_to_io)
  }

  /// Returns whether a password matches an accounts password hash or not.
  fn is_valid_password(&self, password: &str, account: &object::Account) -> io::Result<bool> {
    bcrypt::verify(password, &account.password_hash).map_err(util::bcrypt_to_io)
  }

  /// Returns whether an account is timed out or not.
  fn is_timed_out(&self, account: &object::Account) -> io::Result<bool> {
    match account.failed_login_time {
      None => Ok(false),
      Some(last_time) => {
        let attempts = (account.failed_login_attempts as u32).saturating_sub(self.lockout_attempts);
        let delay = 2u64.pow(attempts).min(self.lockout_time_max);
        Ok(delay > 0 && (util::unix_timestamp()?.as_secs() - last_time as u64) <= delay)
      },
    }
  }

  /// Increases an account's number of failed login attempts.
  fn increment_login_attempts(&self, account: &mut object::Account) -> io::Result<()> {
    account.failed_login_attempts += 1;
    account.failed_login_time = Some(util::unix_timestamp()?.as_secs() as i64);
    self.repository.update(account)
  }

  /// Resets an account's number of failed login attempts.
  fn login_account(&self, account: &mut object::Account) -> io::Result<()> {
    account.failed_login_attempts = 0;
    account.failed_login_time = None;
    account.logged_in = true;
    self.repository.update(account)
  }
}

mod util {
  use super::*;
  use std::convert::TryFrom;
  use std::time::{Duration, SystemTime, UNIX_EPOCH};

  /// Converts a data object to a domain model.
  pub fn map_to_entity(account: object::Account) -> io::Result<Account> {
    Ok(Account {
      id: account.id,
      username: account.username,
      password_hash: account.password_hash,
      security_code: u32::try_from(account.security_code)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "column 'security_code'"))?,
      email: account.email,
    })
  }

  /// Converts a Bcrypt error to an IO error.
  pub fn bcrypt_to_io(error: bcrypt::BcryptError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error)
  }

  /// Returns the current time as a UNIX timestamp.
  pub fn unix_timestamp() -> io::Result<Duration> {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
  }
}
