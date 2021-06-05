//! Support for rescaling quantities.

mod decimal;
mod binary;

pub use decimal::Decimal;
pub use binary::Binary;

/// Trait for an individual scale or prefix.
pub trait Scale {
  /// Get the base for this scale's exponent.
  fn base(&self) -> i32;

  /// Get the exponent for this scale.
  fn exponent(&self) -> i32;

  /// Get the multiplier for this scale as a floating-point number.
  fn multiplier(&self) -> f64 {
    let base = self.base() as f64;
    let exp = self.exponent();
    base.powi(exp)
  }

  /// Get the prefix for this scale.
  fn prefix(&self) -> &'static str;
}

/// Trait for a collection of (related) scales.
pub trait ScaleScheme {
  type Scale;

  /// Get all scales for this scheme.
  fn all_scales() -> &'static [&'static Self::Scale];
}
