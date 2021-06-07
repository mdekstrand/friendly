//! General-purpose quantities with scales.
use crate::scale::*;

/// A numeric quantity to display.
#[derive(Debug, Clone)]
pub struct Quantity<Q, F: PrefixFamily> {
  value: Q,
  scale: Scale<F>
}

impl <Q> Quantity<Q, Decimal> {
  /// Create a new auto-scaled decimal quantity.
  pub fn decimal(value: Q) -> Self {
    Quantity::new(value)
  }
}

impl <Q> Quantity<Q, Binary> {
  /// Create a new auto-scaled binary quantity.
  pub fn binary(value: Q) -> Self {
    Quantity::new(value)
  }
}

impl <Q, F: PrefixFamily> Quantity<Q, F> {
  /// Create a new auto-scaled quantity of arbitrary prefix type.
  pub fn new(value: Q) -> Self {
    Quantity {
      value,
      scale: Scale::Auto
    }
  }

  pub fn get_scale<'a>(&'a self) -> &'a Scale<F> {
    &self.scale
  }

  /// Configure this quantity with a different scale.
  ///
  /// The use of `Into` bounds allows this to rescale with either a fixed scale:
  ///
  /// ```
  /// # use hd::scale::*;
  /// # use hd::quantity::Quantity;
  /// let q = Quantity::decimal(10324);
  /// let q = q.scale(Decimal::KILO);
  /// assert_eq!(q.get_scale(), &Scale::Fixed(Decimal::KILO));
  /// ```
  ///
  /// Or auto-scaling:
  /// ```
  /// # use hd::scale::*;
  /// # use hd::quantity::Quantity;
  /// let q = Quantity::decimal(10324);
  /// let q = q.scale(Decimal::AUTO);
  /// assert_eq!(q.get_scale(), &Scale::Auto);
  /// ```
  pub fn scale<F2: PrefixFamily, S: Into<Scale<F2>>>(self, scale: S) -> Quantity<Q, F2> {
    Quantity {
      value: self.value,
      scale: scale.into(),
    }
  }
}
