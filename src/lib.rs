//! # zigzag-rs
//!
//! Zigzag encoding/decoding for primitive integers.
//!
//! # Examples
//!
//! ```
//! use zigzag_rs::ZigZag;
//!
//! let encoded: u32 = i32::encode(-3);
//! assert_eq!(encoded, 5);
//! assert_eq!(i32::decode(encoded), -3);
//! ```
//!
//! # `no_std`
//!
//! This crate is `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod zigzag;

pub use zigzag::ZigZag;
