//! Human-friendly display library.
//!
//! This module provides a convenient, uniform way to display various types of quantities
//! in approximate, human-readable format.  For example:
//!
//! ```
//! # use friendly::bytes;
//! let kb = format!("{}", bytes(13200));
//! assert_eq!(kb.as_str(), "12.89 KiB")
//! ```
//!
//! The various functions provide quick ways to wrap values and types in the appropriate
//! objects to facilitate their display. Types such as [Quantity] then provide methods to
//! further customize this presentation.
//!
//! ## Features
//!
//! This crate supports some features:
//!
//! - `chrono` — enables support for types from the Chrono crate (currently just [chrono::Duration])

pub mod quantity;
pub mod scale;
pub mod sigfig;
pub mod temporal;

pub use quantity::Quantity;
pub use scale::Scale;
pub use temporal::{duration, seconds};

use quantity::QVal;

/// Display a number of bytes.
///
/// By default, this uses binary prefixes:
///
/// ```
/// # use friendly::bytes;
/// let kb = format!("{}", bytes(13200));
/// assert_eq!(kb.as_str(), "12.89 KiB")
/// ```
///
/// You can also use decimal prefixes:
///
/// ```
/// # use friendly::bytes;
/// # use friendly::scale::*;
/// let kb = format!("{}", bytes(13200).scale(Decimal::AUTO));
/// assert_eq!(kb.as_str(), "13.20 kB")
/// ```
pub fn bytes<V: QVal>(val: V) -> Quantity<V, scale::Binary> {
    Quantity::binary(val).suffix("B")
}

/// An ordinary auto-scaled value.
pub fn scalar<V: QVal>(val: V) -> Quantity<V, scale::Decimal> {
    Quantity::decimal(val)
}
