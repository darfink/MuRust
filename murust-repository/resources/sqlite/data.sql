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

-- Create item definitions for some test items.
INSERT INTO item_definition
  (code, name, equippable_slot, max_durability, width, height, drop_from_monster, drop_level)
VALUES
  (0,    'Kris',           0,  20, 1, 2, 1, 6),
  (1,    'Short Sword',    0,  22, 1, 3, 1, 3),
  (2,    'Rapier',         0,  23, 1, 3, 1, 9),
  (3585, 'Dragon Helm',    2,  68, 2, 2, 1, 57),
  (4097, 'Dragon Armor',   3,  68, 2, 3, 1, 59),
  (4609, 'Dragon Pants',   4,  68, 2, 2, 1, 55),
  (5121, 'Dragon Gloves',  5,  68, 2, 2, 1, 52),
  (5633, 'Dragon Boots',   6,  68, 2, 2, 1, 54),
  (6656, 'Guardian Angel', 8, 255, 1, 1, 1, 23),
  (6657, 'Imp',            8, 255, 1, 1, 1, 28);

-- Create an item instance of a Kris, Short Sword + Dragon Set
INSERT INTO item(id, code, level, durability)
VALUES
  (X'6606af63a93c11e4979700505690798f',    0,  2, 20),
  (X'3f06af63a93c11e4979700505690773f',    1,  3, 22),
  (X'a64f5979c8684d2eb6dc217dd2e5a009', 3585,  3, 55),
  (X'b64f5979c8684d2eb6dc217dd2e5a009', 4097, 13, 55),
  (X'c64f5979c8684d2eb6dc217dd2e5a009', 4609,  5, 55),
  (X'd64f5979c8684d2eb6dc217dd2e5a009', 5121, 11, 54),
  (X'e64f5979c8684d2eb6dc217dd2e5a009', 5633,  7, 55),
  (X'ed38227dcf6a4a18bdb6721b7fb78f9e', 6657,  0, 10);

-- Equip the 'deadbeef' character with the Short Sword
INSERT INTO equipment_item(character_id, item_id, slot)
VALUES
  (1, X'3f06af63a93c11e4979700505690773f', 0),
  (1, X'a64f5979c8684d2eb6dc217dd2e5a009', 2),
  (1, X'b64f5979c8684d2eb6dc217dd2e5a009', 3),
  (1, X'c64f5979c8684d2eb6dc217dd2e5a009', 4),
  (1, X'd64f5979c8684d2eb6dc217dd2e5a009', 5),
  (1, X'e64f5979c8684d2eb6dc217dd2e5a009', 6),
  (1, X'ed38227dcf6a4a18bdb6721b7fb78f9e', 8);

-- Add the Kris to the 'deadbeef' character's inventory
INSERT INTO inventory_item(inventory_id, item_id, slot)
VALUES
  (1, X'6606af63a93c11e4979700505690798f', 0);

INSERT INTO item_eligible_class(item_code, class)
VALUES
  (0, 'DW'), (0, 'DK'), (0, 'FE'), (0, 'MG'), (0, 'DL'),
  (1, 'DW'), (1, 'DK'), (1, 'FE'), (1, 'MG'), (1, 'DL'),
  (2, 'DK'), (2, 'FE'), (2, 'MG'), (2, 'DL'),
  (3585, 'DK'),
  (4097, 'DK'), (4097, 'MG'),
  (4609, 'DK'), (4609, 'MG'),
  (5121, 'DK'), (5121, 'MG'),
  (5633, 'DK'), (5633, 'MG'),
  (6656, 'DW'), (6656, 'DK'), (6656, 'FE'), (6656, 'MG'), (6656, 'DL'),
  (6657, 'DW'), (6657, 'DK'), (6657, 'FE'), (6657, 'MG'), (6657, 'DL');