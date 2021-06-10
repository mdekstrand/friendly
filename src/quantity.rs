//! General-purpose quantities with scales.
use std::f64::NAN;
use std::fmt;
use num_traits::ToPrimitive;

use crate::scale::*;
use crate::sigfig::*;

/// Trait for values for a quantity.
pub trait QVal: fmt::Display {
  /// Convert to a floating-point value.
  fn as_float(&self) -> f64;
}

impl <V: ToPrimitive + fmt::Display> QVal for V {
  fn as_float(&self) -> f64 {
    self.to_f64().unwrap_or(NAN)
  }
}

/// A numeric quantity to display.
#[derive(Debug, Clone)]
pub struct Quantity<Q: QVal, F: PrefixFamily> {
  value: Q,
  scale: Scale<F>,
  sfx_str: &'static str,
  nsig: u32,
}

impl <Q: QVal> Quantity<Q, Decimal> {
  /// Create a new auto-scaled decimal quantity.
  pub fn decimal(value: Q) -> Self {
    Quantity::new(value)
  }
}

impl <Q: QVal> Quantity<Q, Binary> {
  /// Create a new auto-scaled binary quantity.
  pub fn binary(value: Q) -> Self {
    Quantity::new(value)
  }
}

impl <Q: QVal, F: PrefixFamily> Quantity<Q, F> {
  /// Create a new auto-scaled quantity of arbitrary prefix type.
  pub fn new(value: Q) -> Self {
    Quantity {
      value,
      scale: Scale::Auto,
      sfx_str: "",
      nsig: 4
    }
  }

  /// Configure this quantity with a different scale.
  ///
  /// The use of `Into` bounds allows this to rescale with either a fixed scale:
  ///
  /// ```
  /// # use friendly::scale::*;
  /// # use friendly::quantity::Quantity;
  /// let q = Quantity::decimal(10324);
  /// let q = q.scale(Decimal::KILO);
  /// assert_eq!(q.to_string().as_str(), "10.32 k");
  /// ```
  ///
  /// Or auto-scaling:
  /// ```
  /// # use friendly::scale::*;
  /// # use friendly::quantity::Quantity;
  /// let q = Quantity::decimal(10324);
  /// let q = q.scale(Decimal::AUTO);
  /// assert_eq!(q.to_string().as_str(), "10.32 k");
  /// ```
  pub fn scale<F2: PrefixFamily, S: Into<Scale<F2>>>(self, scale: S) -> Quantity<Q, F2> {
    Quantity {
      value: self.value,
      sfx_str: self.sfx_str,
      nsig: self.nsig,
      scale: scale.into(),
    }
  }

  /// Change the unit suffix on this quantity.
  pub fn suffix(self, suffix: &'static str) -> Self {
    Quantity {
      sfx_str: suffix,
      ..self
    }
  }

  /// Change the significant figures on this quantity.
  pub fn sig_figs(self, sf: u32) -> Self {
    Quantity {
      nsig: sf,
      ..self
    }
  }
}

impl <Q: QVal, F: PrefixFamily> fmt::Display for Quantity<Q, F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let scaled = match self.scale {
      Scale::Native => None,
      Scale::Auto => Some(F::autoscale(self.value.as_float())),
      Scale::Fixed(s) => Some((s.scale_value(self.value.as_float()), s)),
    };
    if let Some((sv, scale)) = scaled {
      let (sv, prec) = sigscale(sv, self.nsig as usize);
      write!(f, "{:.*}", prec, sv)?;
      let sl = scale.label();
      if !sl.is_empty() || !self.sfx_str.is_empty() {
        write!(f, " {}{}", sl, self.sfx_str)?;
      }
    } else {
      write!(f, "{}", self.value)?;
      if !self.sfx_str.is_empty() {
        write!(f, " {}", self.sfx_str)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::Quantity;
  use crate::scale::*;

  #[test]
  fn test_decimal_quantity() {
    let tq = Quantity::decimal(10);
    assert_eq!(tq.value, 10);
    assert_eq!(tq.scale, Scale::Auto);
  }

  #[test]
  fn test_zero() {
    let tq = Quantity::decimal(0);
    assert_eq!(tq.to_string().as_str(), "0.0000");
  }

  #[test]
  fn test_zero_sfx() {
    let tq = Quantity::decimal(0).suffix("B");
    assert_eq!(tq.to_string().as_str(), "0.0000 B");
  }

  #[test]
  fn test_zero_sfx_sf() {
    let tq = Quantity::decimal(0).suffix("B").sig_figs(2);
    assert_eq!(tq.to_string().as_str(), "0.00 B");
  }

  #[test]
  fn test_megawatts() {
    let tq = Quantity::decimal(15_250_000.0).suffix("W");
    assert_eq!(tq.to_string().as_str(), "15.25 MW");
  }

  #[test]
  fn test_kibibytes_ps() {
    let tq = Quantity::binary(182_421.0).suffix("B/s");
    assert_eq!(tq.to_string().as_str(), "178.1 KiB/s");
  }
}
