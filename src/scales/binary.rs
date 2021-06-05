use super::{Scale, ScaleScheme};

/// A binary scale.
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

  pub const ALL_SCALES: &'static [&'static Binary] = &[
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

impl Scale for Binary {
  fn base(&self) -> i32 {
    2
  }

  fn exponent(&self) -> i32 {
    self.exp
  }

  fn prefix(&self) -> &'static str {
    self.pfx
  }
}

impl ScaleScheme for Binary {
  type Scale = Binary;

  fn all_scales() -> &'static [&'static Binary] {
    Binary::ALL_SCALES
  }
}
