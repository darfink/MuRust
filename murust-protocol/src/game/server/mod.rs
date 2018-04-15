//! Game Server Packets

use super::{Version, util::serialize_class, VERSION};
use game::models::{CharacterEquipmentSet, Color};
use muonline_packet_serialize::{IntegerBE, IntegerLE, StringFixed, VectorLengthLE};
use murust_data_model::entities::Character;
use murust_data_model::types::{Class, CtlCode, Direction, GuildRole, HeroStatus};
use serde::{Serialize, Serializer};
use std::iter::IntoIterator;
use typenum;

/// `C1:0D` — Multicast text message sent from the server.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// type | `U8` | The message type. | -
/// count | `U8` | The number of times the message is displayed. | -
/// padding | `U8` | Ignored by the client. | -
/// delay | `U16` | The delay of the message. | LE
/// color | `U32` | The color component (ARGB) of the message. | LE
/// speed | `U8` | The speed of the message. | -
/// text | `CHAR(*)` | The message's content. | -
///
/// Only **Custom** uses the `count`, `delay`, `color` and `speed` attributes.
///
/// Type | Display
/// ---- | -------
/// `0` | Alert
/// `1` | Notice
/// `2` | Guild
/// `10-15` | Custom
#[derive(MuPacket, Debug)]
#[packet(kind = "C1", code = "0D")]
pub enum Message {
  /// Displays the message in the center with yellow flickering text.
  Alert(String),
  /// Displays the message in the upper left corner with a blue tone.
  Notice(String),
  /// Displays the message in the center with green flickering text.
  Guild(String),
  /// Displays the message using custom attributes.
  Custom {
    kind: u8,
    color: Color,
    count: u8,
    delay: u16,
    speed: u8,
    message: String,
  },
}

impl Serialize for Message {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    #[derive(Serialize, Debug, Default)]
    struct MessageData {
      kind: u8,
      count: u8,
      padding: u8,
      #[serde(with = "IntegerLE")]
      delay: u16,
      color: Color,
      speed: u8,
      // TODO: Add support for StringDynamic
      // TODO: Allow string references with StringFixed
      #[serde(with = "StringFixed::<typenum::U60>")]
      message: String,
      padding2: u8,
    }

    let mut data = MessageData::default();
    match self {
      // TODO: Refactor this somehowzzz
      &Message::Alert(ref text) => {
        data.kind = 0;
        data.message = text.clone();
      },
      &Message::Notice(ref text) => {
        data.kind = 1;
        data.message = text.clone();
      },
      &Message::Guild(ref text) => {
        data.kind = 2;
        data.message = text.clone();
      },
      &Message::Custom {
        kind,
        color,
        count,
        delay,
        speed,
        ref message,
      } => {
        data.kind = kind + 10;
        data.color = color;
        data.count = count;
        data.delay = delay;
        data.speed = speed;
        data.message = message.clone();
      },
    }

    data.serialize(serializer)
  }
}

/// `C1:F1:00` — Describes the result of an attempt to join a Game Server.
///
/// This can also be sent after a client has connected. If sent after the initial
/// connect, the client ignores the `result` field and updates the saved client
/// ID. It also sends a new [AccountLoginRequest](../client/struct.AccountLoginRequest.html).
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// result | `U8` | Boolean representing success or failure. | -
/// client ID | `U16` | Client ID identifying the user. | BE
/// version | `U8(5)` | Protocol version used for communication. | BE
///
/// ## Example
///
/// ```c
/// [0xC1, 0x05, 0xF1, 0x01, 0x01]
/// ```
#[derive(MuPacket, Debug)]
#[packet(kind = "C1", code = "F1", subcode = "00")]
pub enum JoinResult {
  Success { client_id: u16, version: Version },
  Failure,
}

impl JoinResult {
  /// Creates a new successful join result.
  pub fn success(client_id: u16) -> Self {
    JoinResult::Success {
      client_id,
      version: VERSION,
    }
  }
}

impl Serialize for JoinResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    #[derive(Serialize, Debug)]
    struct JoinResultSuccess {
      result: u8,
      #[serde(with = "IntegerBE")]
      client_id: u16,
      version: Version,
    }

    match self {
      &JoinResult::Failure => 0u8.serialize(serializer),
      &JoinResult::Success { client_id, version } => {
        let data = JoinResultSuccess {
          result: 1,
          client_id,
          version,
        };
        data.serialize(serializer)
      },
    }
  }
}

/// `C1:F1:01` — Describes the result of an account login attempt.
///
/// Describes the result of an attempted login operation.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// result | `U8` | Integer representing the result of a login attempt. | -
///
/// ## Example
///
/// ```c
/// [0xC1, 0x05, 0xF1, 0x01, 0x01]
/// ```
#[repr(u8)]
#[derive(MuPacket, Primitive, Copy, Clone, Debug)]
#[packet(kind = "C1", code = "F1", subcode = "01")]
pub enum AccountLoginResult {
  IncorrectPassword = 0x00,
  Success = 0x01,
  InvalidAccount = 0x02,
  AlreadyConnected = 0x03,
  ServerIsFull = 0x04,
  AccountIsBlocked = 0x05,
  InvalidGameVersion = 0x06,
  TooManyAttempts = 0x08,
  NoPaymentInformation = 0x09,
  SubscriptionIsOver = 0x0A,
  SubscriptionIsOverForIP = 0x0D,
  IneligibleAge = 0x11,
  NoPointsForDate = 0xC0,
  NoPoints = 0xC1,
  BannedIP = 0xC2,
  Error = 0xFF,
}

primitive_serialize!(AccountLoginResult, u8);

/// `C1:F3:00` — Represents a list of available characters.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// limit | `U8` | The maximum class available. | -
/// teleport | `U8` | The character's teleport information. | -
/// count | `U8` | The number of characters in this response. | -
/// characters | `Character[]` | An array of characters. | -
///
/// ### Layout - Character
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// index | `U8` | The character's index. | -
/// name | `CHAR(10)` | The character's name. | -
/// level | `U16` | The character's level. | LE
/// class | `U8` | The character's class. | -
/// EQ | `U8(17)` | The character's equipment. | -
/// CTL | `U8` | The user's CTL code. | -
/// guild | `U8` | The character's guild status. | -
#[derive(Serialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "00")]
pub struct CharacterList {
  max_class: Class,
  teleport: u8,
  #[serde(with = "VectorLengthLE::<u8>")]
  characters: Vec<CharacterListEntry>,
}

impl CharacterList {
  /// Constructs a new character list.
  pub fn new<'a, 'b, I>(max_class: Class, characters: I) -> Self
  where
    I: IntoIterator<Item = (&'a Character)>,
  {
    CharacterList {
      max_class,
      teleport: 0,
      characters: characters
        .into_iter()
        .map(|character| CharacterListEntry {
          slot: character.slot,
          name: character.name.clone(),
          padding: 0,
          level: character.level,
          ctl: CtlCode::None,
          class: character.class,
          equipment: CharacterEquipmentSet::new(&character.equipment),
          guild: GuildRole::None,
        })
        .collect::<Vec<_>>(),
    }
  }
}

impl Default for CharacterList {
  /// An empty character list.
  fn default() -> Self {
    CharacterList {
      max_class: Class::FairyElf,
      teleport: 0,
      characters: Vec::new(),
    }
  }
}

/// A Character list entry.
#[derive(Serialize, Debug)]
struct CharacterListEntry {
  slot: u8,
  #[serde(with = "StringFixed::<typenum::U10>")]
  name: String,
  padding: u8,
  #[serde(with = "IntegerLE")]
  level: u16,
  ctl: CtlCode,
  #[serde(serialize_with = "serialize_class")]
  class: Class,
  equipment: CharacterEquipmentSet,
  guild: GuildRole,
}

/// `C1:F3:01` — Describes a result of a character creation.
///
/// The `FailureOther` field does not present any information to the client, but
/// it's assumed to be accompanied by a server message, informing to user of a
/// different result.
#[derive(MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "01")]
pub enum CharacterCreateResult {
  InvalidName,
  LimitReached,
  FailureOther,
  Character {
    name: String,
    slot: u8,
    level: u16,
    class: Class,
  },
}

impl CharacterCreateResult {
  /// Creates a successful character result.
  pub fn success(character: &Character) -> Self {
    CharacterCreateResult::Character {
      name: character.name.clone(),
      slot: character.slot,
      level: character.level,
      class: character.class,
    }
  }
}

impl Serialize for CharacterCreateResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    #[derive(Serialize)]
    struct CharacterCreateResultSuccess {
      result: u8,
      #[serde(with = "StringFixed::<typenum::U10>")]
      name: String,
      slot: u8,
      #[serde(with = "IntegerLE")]
      level: u16,
      #[serde(serialize_with = "serialize_class")]
      class: Class,
    }

    match self {
      &CharacterCreateResult::InvalidName => 0u8.serialize(serializer),
      &CharacterCreateResult::LimitReached => 2u8.serialize(serializer),
      &CharacterCreateResult::FailureOther => 3u8.serialize(serializer),
      &CharacterCreateResult::Character {
        ref name,
        slot,
        level,
        class,
      } => CharacterCreateResultSuccess {
        result: 1,
        name: name.clone(),
        slot,
        level,
        class,
      }.serialize(serializer),
    }
  }
}

/// `C1:F3:02` — Describes a result of a character deletion.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// result | `U8` | The result of the deletion. | -
///
/// Result | Meaning
/// ------ | -------
/// `0x00` | Can't delete guild character.
/// `0x01` | Successfully deleted character.
/// `0x02` | Invalid personal ID number.
/// `0x03` | The character is item blocked.
#[repr(u8)]
#[derive(MuPacket, Primitive, Copy, Clone, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "02")]
pub enum CharacterDeleteResult {
  GuildCharacter = 0x00,
  Success = 0x01,
  InvalidSecurityCode = 0x02,
  Blocked = 0x03,
}

primitive_serialize!(CharacterDeleteResult, u8);

/// `C1:F3:03` — Describes a character's information.
///
/// Sent upon entering a world after selecting a character.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// x | `U8` | The character's horizontal position. | -
/// y | `U8` | The character's vertical position. | -
/// [world](#world) | `U8` | The character's current world. | -
/// angle | `U8` | The character's angle. | -
/// XP | `U32` | The character's experience. | LE
/// XP (level) | `U32` | The required amount of experience to level up. | LE
/// points | `U16` | The amount of level up points available. | LE
/// strength | `U16` | The character's strength. | LE
/// agility | `U16` | The character's agility. | LE
/// vitality | `U16` | The character's vitality. | LE
/// energy | `U16` | The character's energy. | LE
/// HP | `U16` | The character's HP. | LE
/// HP (max) | `U16` | The character's max HP. | LE
/// MP | `U16` | The character's MP. | LE
/// MP (max) | `U16` | The character's max MP. | LE
/// SD | `U16` | The character's new SD. | LE
/// SD (max) | `U16` | The character's max SD. | LE
/// AG | `U16` | The character's AG. | LE
/// AG (max) | `U16` | The character's max AG. | LE
/// padding | `U16` | Padding, ignored by the client. | -
/// money | `U32` | The amount of zen. | LE
/// PK | `U8` | The character's PK status. | LE
/// CTL | `U8` | The user's CTL code. | LE
/// FP⊕ | `U16` | The character's fruit points (increase). | LE
/// FP⊕(max) | `U16` | The character's max fruit points (increase). | LE
/// command | `U16` | The character's command. | LE
/// FP⊖ | `U16` | The character's fruit points (decrease). | LE
/// FP⊖(max) | `U16` | The character's max fruit points (decrease). | LE
#[derive(Serialize, MuPacket, Debug, Default)]
#[packet(kind = "C1", code = "F3", subcode = "03")]
pub struct CharacterJoin {
  pub x: u8,
  pub y: u8,
  pub world: u8,
  pub direction: Direction,
  #[serde(with = "IntegerLE")]
  pub experience: u32,
  #[serde(with = "IntegerLE")]
  pub experience_level: u32,
  #[serde(with = "IntegerLE")]
  pub points: u16,
  #[serde(with = "IntegerLE")]
  pub strength: u16,
  #[serde(with = "IntegerLE")]
  pub agility: u16,
  #[serde(with = "IntegerLE")]
  pub vitality: u16,
  #[serde(with = "IntegerLE")]
  pub energy: u16,
  #[serde(with = "IntegerLE")]
  pub health: u16,
  #[serde(with = "IntegerLE")]
  pub health_max: u16,
  #[serde(with = "IntegerLE")]
  pub mana: u16,
  #[serde(with = "IntegerLE")]
  pub mana_max: u16,
  #[serde(with = "IntegerLE")]
  pub shield: u16,
  #[serde(with = "IntegerLE")]
  pub shield_max: u16,
  #[serde(with = "IntegerLE")]
  pub ag: u16,
  #[serde(with = "IntegerLE")]
  pub ag_max: u16,
  pub padding: u16,
  #[serde(with = "IntegerLE")]
  pub money: u32,
  pub hero_status: HeroStatus,
  pub ctl: CtlCode,
  #[serde(with = "IntegerLE")]
  pub fruit_points_add: u16,
  #[serde(with = "IntegerLE")]
  pub fruit_points_add_max: u16,
  #[serde(with = "IntegerLE")]
  pub command: u16,
  #[serde(with = "IntegerLE")]
  pub fruit_points_sub: u16,
  #[serde(with = "IntegerLE")]
  pub fruit_points_sub_max: u16,
}
