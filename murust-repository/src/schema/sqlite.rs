/// The SQL file the schema is based on.
pub const DEFAULT: &'static str = include_str!("../../resources/sqlite/schema.sql");

/// The default test data.
pub const TEST_DATA: &'static str = include_str!("../../resources/sqlite/data.sql");

table! {
    account (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        security_code -> Integer,
        email -> Text,
        logged_in -> Bool,
        failed_login_attempts -> Integer,
        failed_login_time -> Nullable<BigInt>,
    }
}

table! {
    character (id) {
        id -> Integer,
        slot -> Integer,
        name -> Text,
        level -> Integer,
        class -> Text,
        experience -> Integer,
        strength -> Integer,
        agility -> Integer,
        vitality -> Integer,
        energy -> Integer,
        command -> Integer,
        map -> Integer,
        position_x -> Integer,
        position_y -> Integer,
        player_kills -> Integer,
        inventory_id -> Integer,
        account_id -> Integer,
    }
}

table! {
    equipment_item (character_id, slot) {
        character_id -> Integer,
        item_id -> Binary,
        slot -> Integer,
    }
}

table! {
    inventory (id) {
        id -> Integer,
        width -> Integer,
        height -> Integer,
        money -> Integer,
    }
}

table! {
    inventory_item (inventory_id, slot) {
        inventory_id -> Integer,
        item_id -> Binary,
        slot -> Integer,
    }
}

table! {
    item (id) {
        id -> Binary,
        level -> Integer,
        durability -> Integer,
        item_code -> Integer,
    }
}

table! {
    item_attribute_boost (item_code, attribute) {
        item_code -> Integer,
        attribute -> Text,
        boost -> Integer,
    }
}

table! {
    item_attribute_requirement (item_code, attribute) {
        item_code -> Integer,
        attribute -> Text,
        requirement -> Integer,
    }
}

table! {
    item_definition (code) {
        code -> Integer,
        name -> Text,
        equippable_slot -> Nullable<Integer>,
        max_durability -> Integer,
        width -> Integer,
        height -> Integer,
        drop_from_monster -> Bool,
        drop_level -> Integer,
    }
}

table! {
    item_eligible_class (item_code, class) {
        item_code -> Integer,
        class -> Text,
    }
}

joinable!(character -> account (account_id));
joinable!(character -> inventory (inventory_id));
joinable!(equipment_item -> character (character_id));
joinable!(equipment_item -> item (item_id));
joinable!(inventory_item -> inventory (inventory_id));
joinable!(inventory_item -> item (item_id));
joinable!(item -> item_definition (item_code));
joinable!(item_attribute_boost -> item_definition (item_code));
joinable!(item_attribute_requirement -> item_definition (item_code));
joinable!(item_eligible_class -> item_definition (item_code));

allow_tables_to_appear_in_same_query!(
  account,
  character,
  equipment_item,
  inventory,
  inventory_item,
  item,
  item_attribute_boost,
  item_attribute_requirement,
  item_definition,
  item_eligible_class,
);
