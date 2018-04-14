use entities::Item;
use enum_map::EnumMap;
use types::ItemSlot;

pub type Equipment = EnumMap<ItemSlot, Option<Item>>;
