use approx::*;
use proptest::prelude::*;
use proptest::sample::*;

use super::binary::Binary;
use super::decimal::Decimal;

use super::*;

pub fn arb_prefix<T: PrefixFamily>() -> Select<&'static T::Prefix> {
    select(T::all_prefixes())
}

proptest! {
  #[test]
  fn test_dec_multiplier(p in arb_prefix::<Decimal>()) {
    assert_eq!(p.multiplier(), (p.base() as f64).powi(p.exponent()));
  }

  #[test]
  fn test_bin_multiplier(p in arb_prefix::<Binary>()) {
    assert_eq!(p.multiplier(), (p.base() as f64).powi(p.exponent()));
  }

  #[test]
  fn test_dec_autoscale(f in any::<f64>()) {
    let (sx, scale) = Decimal::autoscale(f);
    if scale.exponent() > -24 && f.is_normal() {
      assert!(sx.abs() >= 1.0);
    }
    if scale.exponent() < 24 {
      assert!(sx.abs() < 1000.0);
    }
    assert_relative_eq!(sx * scale.multiplier(), f);
  }
}
