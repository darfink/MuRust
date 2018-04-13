use configuration::ItemSlot;
use entities::Item;
use enum_map::EnumMap;

pub type Equipment = EnumMap<ItemSlot, Option<Item>>;
