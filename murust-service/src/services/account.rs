use bcrypt;
use error::{Error, Result};
use mapping::MappableEntity;
use murust_data_model::entities::Account;
use murust_repository::{models, AccountRepository};

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
  ) -> Result<::std::result::Result<Account, AccountLoginError>> {
    // Fetches an account that is not timed out, matching the supplied username
    let mut account = match self.repository.find_by_username(username)? {
      None => return Ok(Err(AccountLoginError::InvalidUsername)),
      Some(account) => account,
    };

    let map_to_entity = |account: models::Account| Account::try_map(account, ());
    let error = if self.is_timed_out(&account)? {
      AccountLoginError::TooManyAttempts(map_to_entity(account)?)
    } else if !self.is_valid_password(password, &account)? {
      self.increment_login_attempts(&mut account)?;
      AccountLoginError::InvalidPassword(map_to_entity(account)?)
    } else if account.logged_in {
      AccountLoginError::AlreadyConnected(map_to_entity(account)?)
    } else {
      self.login_account(&mut account)?;
      return Ok(Ok(map_to_entity(account)?));
    };

    Ok(Err(error))
  }

  /// Logs out an account from the underlying repository.
  pub fn logout(&self, account: &Account) -> Result<()> {
    let mut models = self
      .repository
      .find_by_id(&account.id)?
      .ok_or(Error::MissingPersistence)?;
    models.logged_in = true;
    Ok(self.repository.update(&models)?)
  }

  /// Creates a new account and returns it as an models.
  pub fn create(
    &self,
    username: &str,
    password: &str,
    security_code: u32,
    email: &str,
  ) -> Result<Account> {
    let password_hash = self.hash_password(password)?;
    let account = self
      .repository
      .create(username, &password_hash, security_code as i32, email)?;
    Ok(Account::try_map(account, ())?)
  }

  /// Updates an account's underlying storage.
  pub fn update(&self, account: &Account) -> Result<()> {
    let mut models = self
      .repository
      .find_by_id(&account.id)?
      .ok_or(Error::MissingPersistence)?;

    // TODO: Can these allocations be prevented?
    if account.username != models.username {
      models.username = account.username.clone();
    }
    if account.email != models.email {
      models.email = account.email.clone();
    }

    models.security_code = account.security_code as i32;
    Ok(self.repository.update(&models)?)
  }

  /// Removes an account from the underlying storage.
  pub fn delete(&self, account: Account) -> ::std::result::Result<(), (Error, Account)> {
    self
      .repository
      .delete(&account.id)
      .map_err(|error| (error.into(), account))
  }

  /// Returns the hash of a password.
  fn hash_password(&self, password: &str) -> Result<String> {
    bcrypt::hash(password, self.hashing_cost).map_err(Into::into)
  }

  /// Returns whether a password matches an accounts password hash or not.
  fn is_valid_password(&self, password: &str, account: &models::Account) -> Result<bool> {
    bcrypt::verify(password, &account.password_hash).map_err(Into::into)
  }

  /// Returns whether an account is timed out or not.
  fn is_timed_out(&self, account: &models::Account) -> Result<bool> {
    account.failed_login_time.map_or(Ok(false), |last_time| {
      let attempts = (account.failed_login_attempts as u32).saturating_sub(self.lockout_attempts);
      let delay = 2u64.pow(attempts).min(self.lockout_time_max);
      Ok(delay > 0 && (util::unix_timestamp()?.as_secs() - last_time as u64) <= delay)
    })
  }

  /// Increases an account's number of failed login attempts.
  fn increment_login_attempts(&self, account: &mut models::Account) -> Result<()> {
    account.failed_login_attempts += 1;
    account.failed_login_time = Some(util::unix_timestamp()?.as_secs() as i64);
    Ok(self.repository.update(account)?)
  }

  /// Resets an account's number of failed login attempts.
  fn login_account(&self, account: &mut models::Account) -> Result<()> {
    account.failed_login_attempts = 0;
    account.failed_login_time = None;
    account.logged_in = true;
    Ok(self.repository.update(account)?)
  }
}

mod util {
  use super::*;
  use std::time::{Duration, SystemTime, UNIX_EPOCH};

  /// Returns the current time as a UNIX timestamp.
  pub fn unix_timestamp() -> Result<Duration> {
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(Into::into)
  }
}
