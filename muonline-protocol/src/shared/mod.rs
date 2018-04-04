/// Serial identifier for the client.
pub type Serial = [u8; 16];

/// Version identifier for the protocol.
pub type Version = [u8; 5];

/// The protocol version.
pub static VERSION: Version = [0x30, 0x30, 0x30, 0x30, 0x30];
