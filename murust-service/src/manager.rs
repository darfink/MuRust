use murust_repository::*;
use {AccountService, CharacterService, ItemService};

/// A manager for all services.
pub struct ServiceManager {
  context: DataContext,
}

impl ServiceManager {
  /// Returns a new service manager.
  pub fn new(context: DataContext) -> Self { ServiceManager { context } }

  /// Returns the account service.
  pub fn account_service(&self) -> AccountService {
    AccountService::new(AccountRepository::new(&self.context))
  }

  /// Returns the item service.
  pub fn item_service(&self) -> ItemService {
    ItemService::new(
      ItemRepository::new(&self.context),
      ItemDefinitionRepository::new(&self.context),
      ItemEligibleClassRepository::new(&self.context),
    )
  }

  /// Returns the character service.
  pub fn character_service(&self) -> CharacterService {
    CharacterService::new(
      self.item_service(),
      CharacterRepository::new(&self.context),
      InventoryRepository::new(&self.context),
    )
  }
}
