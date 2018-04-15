use muonline_packet_serialize::{StringFixedTransform, StringTransform};
use murust_data_model::types::Class;
use num_traits::FromPrimitive;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

/// A transform for credentials.
pub struct TransformCredentials(());

impl StringTransform for TransformCredentials {
  /// Encrypts or decrypts credentials using an XOR cipher.
  fn process(bytes: &mut [u8]) {
    const CIPHER: [u8; 3] = [0xFC, 0xCF, 0xAB];

    for (byte, xor) in bytes.iter_mut().zip(CIPHER.iter().cycle()) {
      *byte ^= *xor;
    }
  }
}

/// Shorthand alias for encrypted credentials.
pub type StringFixedCredentials<S> = StringFixedTransform<S, TransformCredentials>;

/// Serializes a class value encoded as expected by the client.
pub fn serialize_class<S>(class: &Class, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let class = *class as u8;
  ((class << 5) | ((class & 0x08) << 1)).serialize(serializer)
}

/// Deserializes a class value encoded by the client.
pub fn deserialize_class<'de, D>(deserializer: D) -> Result<Class, D::Error>
where
  D: Deserializer<'de>,
{
  u8::deserialize(deserializer).and_then(|value| {
    Class::from_u8(value >> 4).ok_or_else(|| {
      serde::de::Error::invalid_value(
        serde::de::Unexpected::Other("integer value"),
        &"a valid integer range",
      )
    })
  })
}
