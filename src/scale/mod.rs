//! Support for rescaling quantities.
use std::fmt::Debug;

mod binary;
mod decimal;
#[cfg(test)]
pub(crate) mod test;

pub use binary::Binary;
pub use decimal::Decimal;

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
    type Prefix: Prefix + 'static;

    fn unit_prefix() -> Self::Prefix;

    /// Get all prefixes for this scheme.  The prefixes must be in sorted order.
    fn all_prefixes() -> &'static [&'static Self::Prefix];

    /// Auto-scale a value.
    fn autoscale(val: f64) -> (f64, Self::Prefix) {
        if !val.is_finite() || !val.is_normal() {
            // non-finite values just get displayed, as does ~0
            return (val, Self::unit_prefix());
        }

        let pfxs = Self::all_prefixes();
        let mut iter = pfxs.iter();
        // always have at least one
        let mut cur = iter.next().unwrap();
        while let Some(next) = iter.next() {
            // check fit w.r.t. next
            if next.scale_value(val).abs() < 1.0 {
                // next is too small, 'cur' is what we want
                break;
            } else {
                // save and let's try the next value
                cur = next;
            }
        }

        (cur.scale_value(val), **cur)
    }
}

/// A scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale<F: PrefixFamily> {
    /// Auto-scale to the best-fitting prefix.
    Auto,
    /// Display the number in its native scale.
    Native,
    /// Use a specific fixed prefix.
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
