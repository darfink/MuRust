CREATE TABLE IF NOT EXISTS account(
  id INTEGER NOT NULL,
  username TEXT NOT NULL UNIQUE CHECK(LENGTH(username) <= 10),
  password_hash TEXT NOT NULL CHECK(LENGTH(password_hash) == 60),
  security_code INTEGER NOT NULL CHECK(LENGTH(security_code) <= 7 AND security_code >= 0),
  email TEXT NOT NULL UNIQUE,
  logged_in TINYINT NOT NULL DEFAULT 0 CHECK(logged_in IN (0, 1)),
  failed_login_attempts INTEGER NOT NULL DEFAULT 0 CHECK(failed_login_attempts >= 0),
  failed_login_time BIGINT,
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS character(
  id INTEGER NOT NULL,
  slot INTEGER NOT NULL CHECK(slot BETWEEN 0 AND 4),
  name TEXT NOT NULL CHECK(LENGTH(name) <= 10),
  level INTEGER NOT NULL DEFAULT 1 CHECK(level BETWEEN 1 AND 0xFFFF),
  class TEXT NOT NULL CHECK(class IN ('DW', 'DK', 'FE', 'MG', 'DL', 'SM', 'BK', 'ME')),
  experience INTEGER NOT NULL DEFAULT 0 CHECK(experience >= 0),
  strength INTEGER NOT NULL DEFAULT 0 CHECK(strength BETWEEN 0 AND 0xFFFF),
  agility INTEGER NOT NULL DEFAULT 0 CHECK(agility BETWEEN 0 AND 0xFFFF),
  vitality INTEGER NOT NULL DEFAULT 0 CHECK(vitality BETWEEN 0 AND 0xFFFF),
  energy INTEGER NOT NULL DEFAULT 0 CHECK(energy BETWEEN 0 AND 0xFFFF),
  command INTEGER NOT NULL DEFAULT 0 CHECK(command BETWEEN 0 AND 0xFFFF),
  map INTEGER NOT NULL CHECK(map BETWEEN 0 AND 0xFF),
  position_x INTEGER NOT NULL CHECK(position_x BETWEEN 0 AND 0xFF),
  position_y INTEGER NOT NULL CHECK(position_y BETWEEN 0 AND 0xFF),
  player_kills INTEGER NOT NULL DEFAULT 0,
  inventory_id INTEGER NOT NULL,
  account_id INTEGER NOT NULL,
  UNIQUE(name),
  UNIQUE(account_id, slot),
  FOREIGN KEY(inventory_id) REFERENCES inventory(id),
  FOREIGN KEY(account_id) REFERENCES account(id),
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS inventory(
  id INTEGER NOT NULL,
  width INTEGER NOT NULL CHECK(width BETWEEN 1 AND 0xFF),
  height INTEGER NOT NULL CHECK(height BETWEEN 1 AND 0xFF),
  money INTEGER NOT NULL DEFAULT 0 CHECK(money >= 0),
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS inventory_item(
  inventory_id INTEGER NOT NULL,
  item_id BINARY NOT NULL,
  slot INTEGER NOT NULL CHECK(slot BETWEEN 0 AND 0xFF),
  FOREIGN KEY(inventory_id) REFERENCES inventory(id),
  FOREIGN KEY(item_id) REFERENCES item(id),
  PRIMARY KEY(inventory_id, slot)
);

CREATE TABLE IF NOT EXISTS equipment_item(
  character_id INTEGER NOT NULL,
  item_id BINARY NOT NULL,
  slot INTEGER NOT NULL CHECK(slot BETWEEN 0 AND 11),
  FOREIGN KEY(character_id) REFERENCES character(id),
  FOREIGN KEY(item_id) REFERENCES item(id),
  PRIMARY KEY(character_id, slot)
);

CREATE TABLE IF NOT EXISTS item(
  id BINARY NOT NULL CHECK(TYPEOF(id) = 'blob' AND LENGTH(id) = 16),
  level INTEGER NOT NULL DEFAULT 0 CHECK(level BETWEEN 0 AND 15),
  durability INTEGER NOT NULL CHECK(durability BETWEEN 0 AND 0xFF),
  item_definition_id INTEGER NOT NULL,
  FOREIGN KEY(item_definition_id) REFERENCES item_definition(id),
  PRIMARY KEY(id)
);

-- Add excellent, option, skill & luck
CREATE TABLE IF NOT EXISTS item_definition(
  id INTEGER NOT NULL,
  name TEXT NOT NULL,
  `group` INTEGER NOT NULL CHECK(`group` BETWEEN 0 AND 15),
  `index` INTEGER NOT NULL CHECK(`index` BETWEEN 0 AND 0x1FF),
  modifier INTEGER NOT NULL DEFAULT 0 CHECK(modifier BETWEEN 0 AND 15),
  equippable_slot INTEGER CHECK(IFNULL(equippable_slot, 0) BETWEEN 0 AND 11),
  max_durability INTEGER NOT NULL CHECK(max_durability BETWEEN 0 AND 0xFF),
  width INTEGER NOT NULL DEFAULT 1 CHECK(width BETWEEN 1 AND 8),
  height INTEGER NOT NULL DEFAULT 1 CHECK(height BETWEEN 1 AND 8),
  drop_from_monster TINYINT NOT NULL CHECK(drop_from_monster IN (0, 1)),
  drop_level INTEGER NOT NULL CHECK(drop_level BETWEEN 1 AND 0xFFFF),
  UNIQUE(name),
  UNIQUE(`group`, `index`, modifier),
  PRIMARY KEY(id)
);

-- Whitelist of classes able to use an item
CREATE TABLE IF NOT EXISTS item_eligible_class(
  item_definition_id INTEGER NOT NULL,
  class TEXT NOT NULL CHECK(class IN ('DW', 'DK', 'FE', 'MG', 'DL', 'SM', 'BK', 'ME')),
  FOREIGN KEY(item_definition_id) REFERENCES item_definition(id),
  PRIMARY KEY(item_definition_id, class)
);

-- Whitelist of requirements for an item
CREATE TABLE IF NOT EXISTS item_attribute_requirement(
  item_definition_id INTEGER NOT NULL,
  attribute TEXT NOT NULL,
  requirement INTEGER NOT NULL,
  FOREIGN KEY(item_definition_id) REFERENCES item_definition(id),
  PRIMARY KEY(item_definition_id, attribute)
);

-- attribute power-ups from an item
CREATE TABLE IF NOT EXISTS item_attribute_boost(
  item_definition_id INTEGER NOT NULL,
  attribute TEXT NOT NULL,
  boost INTEGER NOT NULL,
  FOREIGN KEY(item_definition_id) REFERENCES item_definition(id),
  PRIMARY KEY(item_definition_id, attribute)
);