use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A Game Server's load balance.
#[derive(Copy, Clone, Debug)]
pub enum ServerLoad {
  /// The server is currently preparing.
  IsPreparing,
  /// The server's current load balance `[0, 1.0]`.
  Load(f32),
}

/// The highest bit indicates whether the server is preparing or not.
const IS_PREPARING: u8 = (1 << 7);

impl From<f32> for ServerLoad {
  fn from(load: f32) -> Self { ServerLoad::Load(load) }
}

impl From<ServerLoad> for u8 {
  fn from(load: ServerLoad) -> Self {
    match load {
      ServerLoad::IsPreparing => IS_PREPARING,
      ServerLoad::Load(value) => (value * 100.0) as u8,
    }
  }
}

impl From<u8> for ServerLoad {
  fn from(load: u8) -> Self {
    if load & IS_PREPARING == IS_PREPARING {
      ServerLoad::IsPreparing
    } else {
      ServerLoad::Load(load as f32 / 100.0 as f32)
    }
  }
}

impl Serialize for ServerLoad {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(self.clone().into())
  }
}

impl<'de> Deserialize<'de> for ServerLoad {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    u8::deserialize(deserializer).map(|v| v.into())
  }
}
