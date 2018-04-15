//! Game Client Packets

pub use self::group::Client;
use super::{Serial, Version, util::deserialize_class};
use game::util::StringFixedCredentials;
use muonline_packet_serialize::{IntegerLE, StringFixed};
use murust_data_model::types::Class;
use std::io;
use typenum;

mod group;

/// `C1:0E:00` - Local client timing values.
///
/// This is sent by default every 20th second.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// time | `U32` | The client's time instant in milliseconds. | LE
/// speed (attack) | `U16` | The client's current attack speed. | LE
/// speed (magic) | `U16` | The client's current magic speed. | LE
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "0E", subcode = "00")]
pub struct ClientTime {
  #[serde(with = "IntegerLE")]
  pub time: u32,
  #[serde(with = "IntegerLE")]
  pub attack_speed: u16,
  #[serde(with = "IntegerLE")]
  pub magic_speed: u16,
}

/// `C1:F1:01` - Authentication request sent upon client login.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// username | `CHAR(10)` | The specified username. | -
/// password | `CHAR(10)` | The specified password. | -
/// time | `U32` | The client's time instant in milliseconds. | LE
/// version | `U8(5)` | The client's protocol version. | -
/// serial | `CHAR(16)` | The client's serial version. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F1", subcode = "01")]
pub struct AccountLoginRequest {
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub username: String,
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub password: String,
  #[serde(with = "IntegerLE")]
  pub time: u32,
  pub version: Version,
  pub serial: Serial,
}

/// `C1:F3:00` - Request for an account's characters.
///
/// This is sent from the client as soon as it has successfully logged in with an
/// account.
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0xF3, 0x00]
/// ```
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "00")]
pub struct CharacterListRequest;

/// `C1:F3:01` - Request for a character creation.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
/// class | `U8` | The character's class. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "01")]
pub struct CharacterCreate {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
  #[serde(deserialize_with = "deserialize_class")]
  pub class: Class,
}

/// `C1:F3:02` - Request for a character deletion.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
/// code | `CHAR(10)` | The account's security code. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "02")]
pub struct CharacterDelete {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub security_code: String,
}

/// `C1:F3:03` - Request for a character selection.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "03")]
pub struct CharacterJoinRequest {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
}
