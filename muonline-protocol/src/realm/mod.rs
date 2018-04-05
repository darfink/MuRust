use muserialize::IntegerBE;
use serde::{Serialize, Serializer};
use shared::{Version, VERSION};

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
