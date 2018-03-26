//! Mu Online Season 2 Protocol.
//!
//! ## Introduction
//!
//! This is an implementation of the [Mu Online](https://en.wikipedia.org/wiki/Mu_Online)
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

extern crate typenum;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate muonline_packet_derive;
extern crate muonline_packet;
extern crate muonline_packet_serialize as muserialize;

pub use self::client::Client;
pub use self::model::*;

pub mod client;
pub mod join;
mod model;
