use configuration::{Class, ItemSlot, ItemTypeId};

#[derive(Debug)]
pub struct ItemDefinition {
  pub id: ItemTypeId,
  pub name: String,
  pub equippable_slot: Option<ItemSlot>,
  pub max_durability: u8,
  pub width: u8,
  pub height: u8,
  pub drop_from_monster: bool,
  pub drop_level: u16,
  pub eligible_classes: Vec<Class>,
}

impl ItemDefinition {
  pub fn new<S: Into<String>>(id: ItemTypeId, name: S) -> Self {
    ItemDefinition {
      id,
      name: name.into(),
      equippable_slot: None,
      max_durability: 0,
      width: 1,
      height: 1,
      drop_from_monster: false,
      drop_level: 0,
      eligible_classes: Vec::new(),
    }
  }
}
