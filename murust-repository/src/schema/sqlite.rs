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
        item_id -> Integer,
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
        item_id -> Integer,
        slot -> Integer,
    }
}

table! {
    item (id) {
        id -> Integer,
        level -> Integer,
        durability -> Integer,
        item_definition_id -> Integer,
    }
}

table! {
    item_attribute_boost (item_definition_id, attribute) {
        item_definition_id -> Integer,
        attribute -> Text,
        boost -> Integer,
    }
}

table! {
    item_attribute_requirement (item_definition_id, attribute) {
        item_definition_id -> Integer,
        attribute -> Text,
        requirement -> Integer,
    }
}

table! {
    item_definition (id) {
        id -> Integer,
        name -> Text,
        group -> Integer,
        index -> Integer,
        modifier -> Integer,
        equippable_slot -> Nullable<Integer>,
        max_durability -> Integer,
        width -> Integer,
        height -> Integer,
        drop_from_monster -> Bool,
        drop_level -> Integer,
    }
}

table! {
    item_eligible_class (item_definition_id, class) {
        item_definition_id -> Integer,
        class -> Text,
    }
}

joinable!(character -> account (account_id));
joinable!(character -> inventory (inventory_id));
joinable!(equipment_item -> character (character_id));
joinable!(equipment_item -> item (item_id));
joinable!(inventory_item -> inventory (inventory_id));
joinable!(inventory_item -> item (item_id));
joinable!(item -> item_definition (item_definition_id));
joinable!(item_attribute_boost -> item_definition (item_definition_id));
joinable!(item_attribute_requirement -> item_definition (item_definition_id));
joinable!(item_eligible_class -> item_definition (item_definition_id));

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
