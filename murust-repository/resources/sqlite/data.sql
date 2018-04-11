PRAGMA foreign_keys = ON;

-- Create a default account used for testing
INSERT INTO account
  (id, username, password_hash, security_code, email)
VALUES
  -- The password is 'test'
  (1, 'foobar', '$2y$07$zFM0q8YmKjaYW4Hig6AFz.wroa/eG5DSK4ST9Y0KS4hDw5Jepw31a', 111111, 'test@mail.com');

-- Create a default character inventory with 8x8 space and 1337 in cash
INSERT INTO inventory(id, width, height, money) VALUES(1, 8, 8, 1337);

-- Create a default DK character named deadbeef at level 3
INSERT INTO character
  (id, slot, name, level, class, map, position_x, position_y, inventory_id, account_id)
VALUES
  (1, 2, 'deadbeef', 3, 'DK', 1, 120, 60, 1, 1);

-- Create item definitions for 'Short Sword' & 'Kris' (incomplete)
INSERT INTO item_definition
  (id, name, `group`, `index`, modifier, equippable_slot, max_durability, width, height, drop_from_monster, drop_level)
VALUES
  (1, 'Kris',        0, 0, 0, 0, 20, 1, 2, 1, 6),
  (2, 'Short Sword', 0, 1, 0, 0, 22, 1, 3, 1, 3);

-- Create an item instance of a 'Short Sword + 3'
INSERT INTO item(id, level, durability, item_definition_id)
VALUES
  (X'6606af63a93c11e4979700505690798f', 2, 20, 1),
  (X'3f06af63a93c11e4979700505690773f', 3, 22, 2);

-- Equip the 'deadbeef' character with the Short Sword
INSERT INTO equipment_item(character_id, item_id, slot)
VALUES
  (1, X'3f06af63a93c11e4979700505690773f', 0);