use super::{Scale, Prefix, PrefixFamily};

/// A binary scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binary {
  pfx: &'static str,
  exp: i32,
}

impl Binary {
  const fn new(pfx: &'static str, exp: i32) -> Binary {
    Binary {
      pfx, exp
    }
  }

  pub const UNIT: Binary = Binary::new("", 0);
  pub const KIBI: Binary = Binary::new("Ki", 10);
  pub const MEBI: Binary = Binary::new("Mi", 20);
  pub const GIBI: Binary = Binary::new("Gi", 30);
  pub const TEBI: Binary = Binary::new("Ti", 40);
  pub const PEBI: Binary = Binary::new("Pi", 50);
  pub const EXBI: Binary = Binary::new("Ei", 60);
  pub const ZEBI: Binary = Binary::new("Zi", 70);
  pub const YOBI: Binary = Binary::new("Yi", 80);

  pub const AUTO: Scale<Binary> = Scale::Auto;

  pub const ALL_PREFIXES: &'static [&'static Binary] = &[
    &Binary::UNIT,
    &Binary::KIBI,
    &Binary::MEBI,
    &Binary::GIBI,
    &Binary::TEBI,
    &Binary::PEBI,
    &Binary::EXBI,
    &Binary::ZEBI,
    &Binary::YOBI,
  ];
}

impl Prefix for Binary {
  #[inline]
  fn base(&self) -> i32 {
    2
  }

  #[inline]
  fn exponent(&self) -> i32 {
    self.exp
  }

  fn multiplier(&self) -> f64 {
    let mult = 1u128 << self.exp;
    mult as f64
  }

  fn label(&self) -> &'static str {
    self.pfx
  }
}

impl PrefixFamily for Binary {
  type Prefix = Binary;

  fn unit_prefix() -> Binary {
    Binary::UNIT
  }

  fn all_prefixes() -> &'static [&'static Binary] {
    Binary::ALL_PREFIXES
  }
}

#[test]
fn test_multipliers() {
  assert_eq!(Binary::UNIT.multiplier(), 1.0);
  assert_eq!(Binary::KIBI.multiplier(), 1024.0);
  assert_eq!(Binary::MEBI.multiplier(), 1024.0 * 1024.0);
  assert_eq!(Binary::GIBI.multiplier(), 1024.0 * 1024.0 * 1024.0);
}
