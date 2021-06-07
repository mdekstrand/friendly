use super::{Scale, Prefix, PrefixFamily};

/// A decimal scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal {
  pfx: &'static str,
  exp: i32,
}

impl Decimal {
  /// Create a new decimal prefix.
  const fn new(pfx: &'static str, exp: i32) -> Decimal {
    Decimal {
      pfx, exp
    }
  }

  pub const YOCTO: Decimal = Decimal::new("y", -24);
  pub const ZEPTO: Decimal = Decimal::new("z", -21);
  pub const ATTO: Decimal = Decimal::new("a", -18);
  pub const FEMTO: Decimal = Decimal::new("f", -15);
  pub const PICO: Decimal = Decimal::new("p", -12);
  pub const NANO: Decimal = Decimal::new("n", -9);
  pub const MICRO: Decimal = Decimal::new("μ", -6);
  pub const MILLI: Decimal = Decimal::new("m", -3);
  pub const UNIT: Decimal = Decimal::new("", 0);
  pub const KILO: Decimal = Decimal::new("k", 3);
  pub const MEGA: Decimal = Decimal::new("M", 6);
  pub const GIGA: Decimal = Decimal::new("G", 9);
  pub const TERA: Decimal = Decimal::new("T", 12);
  pub const PETA: Decimal = Decimal::new("P", 15);
  pub const EXA: Decimal = Decimal::new("E", 18);
  pub const ZETTA: Decimal = Decimal::new("Z", 21);
  pub const YOTTA: Decimal = Decimal::new("Y", 24);

  pub const AUTO: Scale<Decimal> = Scale::Auto;

  pub const ALL_PREFIXES: &'static [&'static Decimal] = &[
    &Decimal::YOCTO,
    &Decimal::ZEPTO,
    &Decimal::ATTO,
    &Decimal::FEMTO,
    &Decimal::PICO,
    &Decimal::NANO,
    &Decimal::MICRO,
    &Decimal::MILLI,
    &Decimal::UNIT,
    &Decimal::KILO,
    &Decimal::MEGA,
    &Decimal::GIGA,
    &Decimal::TERA,
    &Decimal::PETA,
    &Decimal::EXA,
    &Decimal::ZETTA,
    &Decimal::YOTTA,
  ];
}

impl Prefix for Decimal {
  #[inline]
  fn base(&self) -> i32 {
    10
  }

  #[inline]
  fn exponent(&self) -> i32 {
    self.exp
  }

  fn label(&self) -> &'static str {
    self.pfx
  }
}

impl PrefixFamily for Decimal {
  type Prefix = Decimal;

  fn all_prefixes() -> &'static [&'static Decimal] {
    Decimal::ALL_PREFIXES
  }
}

#[test]
fn test_multipliers() {
  assert_eq!(Decimal::UNIT.multiplier(), 1.0);
  assert_eq!(Decimal::KILO.multiplier(), 1000.0);
  assert_eq!(Decimal::MEGA.multiplier(), 1_000_000.0);
  assert_eq!(Decimal::GIGA.multiplier(), 1_000_000_000.0);
}

#[test]
fn test_unit_adjust() {
  assert_eq!(Decimal::UNIT.scale_value(1250), 1250.0);
}

#[test]
fn test_kilo_adjust() {
  assert_eq!(Decimal::KILO.scale_value(1250), 1.250);
}
