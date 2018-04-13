use murust_repository::*;
use {AccountService, CharacterService, ItemService};

pub struct ServiceManager {
  context: DataContext,
}

impl ServiceManager {
  pub fn new(context: DataContext) -> Self { ServiceManager { context } }

  pub fn account_service(&self) -> AccountService {
    AccountService::new(AccountRepository::new(&self.context))
  }

  pub fn item_service(&self) -> ItemService {
    ItemService::new(
      ItemRepository::new(&self.context),
      ItemDefinitionRepository::new(&self.context),
      ItemEligibleClassRepository::new(&self.context),
    )
  }

  pub fn character_service(&self) -> CharacterService {
    CharacterService::new(
      self.item_service(),
      CharacterRepository::new(&self.context),
      InventoryRepository::new(&self.context),
    )
  }
}
