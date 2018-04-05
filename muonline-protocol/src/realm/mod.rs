//! Game Server Packets

use self::meta::CharacterListEntry;
use model::Color;
use muserialize::{IntegerBE, IntegerLE, StringFixed, VectorLengthLE};
use serde::{Serialize, Serializer};
use shared::{Version, VERSION};
use {mu, typenum};

pub mod meta;

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
  pub max_class: mu::Class,
  pub teleport: u8,
  #[serde(with = "VectorLengthLE::<u8>")]
  pub characters: Vec<CharacterListEntry>,
}

impl Default for CharacterList {
  /// The default for a new account.
  fn default() -> Self {
    CharacterList {
      max_class: mu::Class::MagicGladiator,
      teleport: 0,
      characters: Vec::new(),
    }
  }
}
