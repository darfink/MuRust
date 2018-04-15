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
#![recursion_limit = "1024"]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate typenum;

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate muonline_packet;
#[macro_use]
extern crate muonline_packet_serialize;
#[macro_use]
extern crate muonline_packet_derive;

// Used for interoperability between the protocol and entities
extern crate murust_data_model;

pub mod connect;
pub mod game;
