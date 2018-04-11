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
  (code, name, equippable_slot, max_durability, width, height, drop_from_monster, drop_level)
VALUES
  (0, 'Kris',        0, 20, 1, 2, 1, 6),
  (1, 'Short Sword', 0, 22, 1, 3, 1, 3),
  (2, 'Rapier',      0, 23, 1, 3, 1, 9);

-- Create an item instance of a 'Kris + 2' & 'Short Sword + 3'
INSERT INTO item(id, level, durability, item_definition_code)
VALUES
  (X'6606af63a93c11e4979700505690798f', 2, 20, 0),
  (X'3f06af63a93c11e4979700505690773f', 3, 22, 1);

-- Equip the 'deadbeef' character with the Short Sword
INSERT INTO equipment_item(character_id, item_id, slot)
VALUES
  (1, X'3f06af63a93c11e4979700505690773f', 0);

-- Add the Kris to the 'deadbeef' character's inventory
INSERT INTO inventory_item(inventory_id, item_id, slot)
VALUES
  (1, X'6606af63a93c11e4979700505690798f', 0);

INSERT INTO item_eligible_class(item_definition_code, class)
VALUES
  (0, 'DW'), (0, 'DK'), (0, 'FE'), (0, 'MG'), (0, 'DL'),
  (1, 'DW'), (1, 'DK'), (1, 'FE'), (1, 'MG'), (1, 'DL'),
  (2, 'DK'), (2, 'FE'), (2, 'MG'), (2, 'DL');