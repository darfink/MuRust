//! Mu Online Season 2 Protocol.
//!
//! ## Introduction
//!
//! An implementation of the [Mu Online](https://en.wikipedia.org/wiki/Mu_Online)
//! network protocol (season 2, version **1.02c**), used for communication
//! between clients and servers.
//!
//! Mu Online uses a binary protcol consisting of packets with variable size.
//! These packets are commonly encrypted with XOR ciphers and/or symmetric
//! key-algorithms.
//!
//! ## Packet
//!
//! ### Encryption
#![feature(slice_patterns)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate typenum;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate muonline_packet_derive;
extern crate muonline_packet;

#[macro_use]
extern crate muonline_packet_serialize as muserialize;

pub use self::client::Client;
use muserialize::{StringFixedTransform, StringTransform};

pub mod client;
pub mod join;
pub mod realm;
pub mod shared;

/// Shorthand alias for encrypted credentials.
type StringFixedCredentials<S> = StringFixedTransform<S, TransformCredentials>;

/// A transform for credentials.
struct TransformCredentials(());

impl StringTransform for TransformCredentials {
  /// Encrypts or decrypts credentials using an XOR cipher.
  fn process(bytes: &mut [u8]) {
    const CIPHER: [u8; 3] = [0xFC, 0xCF, 0xAB];

    for (byte, xor) in bytes.iter_mut().zip(CIPHER.iter().cycle()) {
      *byte ^= *xor;
    }
  }
}
