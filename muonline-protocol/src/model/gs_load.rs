use serde::{Serialize, Serializer, Deserialize, Deserializer};

/// A Game Server's load balance.
#[derive(Copy, Clone, Debug)]
pub enum GameServerLoad {
  /// The server is currently preparing.
  IsPreparing,
  /// The server's current load balance `[0, 1.0]`.
  Load(f32),
}

/// The highest bit indicates whether the server is preparing or not.
const IS_PREPARING: u8 = (1 << 7);

impl From<GameServerLoad> for u8 {
  fn from(load: GameServerLoad) -> Self {
    match load {
      GameServerLoad::IsPreparing => IS_PREPARING,
      GameServerLoad::Load(value) => (value * 100.0) as u8,
    }
  }
}

impl From<u8> for GameServerLoad {
  fn from(load: u8) -> Self {
    if load & IS_PREPARING == IS_PREPARING {
      GameServerLoad::IsPreparing
    } else {
      GameServerLoad::Load(load as f32 / 100.0 as f32)
    }
  }
}

impl Serialize for GameServerLoad {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_u8(self.clone().into())
  }
}

impl<'de> Deserialize<'de> for GameServerLoad {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    u8::deserialize(deserializer).map(|v| v.into())
  }
}
