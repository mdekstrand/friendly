//! Support for rescaling quantities.
use std::fmt::Debug;

mod decimal;
mod binary;

pub use decimal::Decimal;
pub use binary::Binary;

/// Trait for an individual prefix.
pub trait Prefix: Debug + Clone + Copy + PartialEq {
  /// Get the base for this prefix's exponent.
  fn base(&self) -> i32;

  /// Get the exponent for this prefix.
  fn exponent(&self) -> i32;

  /// Get the multiplier for this prefix as a floating-point number.
  fn multiplier(&self) -> f64 {
    let base = self.base() as f64;
    let exp = self.exponent();
    base.powi(exp)
  }

  /// Rescale a value for this prefix.
  fn scale_value<T: Into<f64>>(&self, value: T) -> f64 {
    let x: f64 = value.into();
    x / self.multiplier()
  }

  /// Get the label for this prefix.
  fn label(&self) -> &'static str;
}

/// Trait for a collection of (related) prefixes.
pub trait PrefixFamily {
  type Prefix: Prefix;

  /// Get all prefixes for this scheme.  The prefixes must be in sorted order.
  fn all_prefixes() -> &'static [&'static Self::Prefix];
}

/// A scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale<F: PrefixFamily> {
  Auto,
  Fixed(F::Prefix),
}

impl From<Decimal> for Scale<Decimal> {
  fn from(p: Decimal) -> Scale<Decimal> {
    Scale::Fixed(p)
  }
}

impl From<Binary> for Scale<Binary> {
  fn from(p: Binary) -> Scale<Binary> {
    Scale::Fixed(p)
  }
}
