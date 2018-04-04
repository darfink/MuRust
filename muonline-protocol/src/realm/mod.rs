use muserialize::IntegerBE;
use serde::{Serialize, Serializer};
use shared::{Version, VERSION};

/// `C1:F1:00` — Describes the result of an attempt to join a game server.
///
/// This can also be sent after a client has connected. If sent after the initial
/// connect, the client ignores the `result` field and updates the saved client
/// ID. It also sends a new [LoginRequest](./struct.LoginRequest.html).
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
    JoinResult::Success { client_id, version: VERSION }
  }
}

impl Serialize for JoinResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
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
        let data = JoinResultSuccess { result: 1, client_id, version };
        data.serialize(serializer)
      },
    }
  }
}
