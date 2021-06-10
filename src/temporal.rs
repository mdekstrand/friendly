//! Human-readable time features.
use std::fmt;
use std::time::Duration;
#[cfg(feature="chrono")]
use chrono;

use crate::scalar;

const MIN_SECS: f64 = 60.0;
const HOUR_SECS: f64 = MIN_SECS * 60.0;
const DAY_SECS: f64 = HOUR_SECS * 24.0;
const WEEK_SECS: f64 = DAY_SECS * 7.0;

/// Human-displayable durations (from [Duration]).
///
/// There are two settings to tweak on a displayable duration:
///
/// - Whether it is full (“3 hours 2 minutes 3.42 seconds”) or compact (“2h2m3.42s”)
/// - How many components are displayed (e.g. with 3 parts, “5d3h2m” will omit seconds)
///
/// The default is compact display with 3 parts.
pub struct HumanDuration {
  seconds: f64,
  compact: bool,
  parts: i32,
}

impl From<Duration> for HumanDuration {
  fn from(d: Duration) -> HumanDuration {
    HumanDuration::new_from_secs(d.as_secs_f64())
  }
}

#[cfg(feature="chrono")]
impl From<chrono::Duration> for HumanDuration {
  fn from(d: chrono::Duration) -> HumanDuration {
    seconds(d.num_milliseconds() as f64 * 0.001)
  }
}

impl HumanDuration {
  /// Create a new readable duration from seconds.
  pub fn new_from_secs(seconds: f64) -> HumanDuration {
    HumanDuration {
      seconds,
      compact: true,
      parts: 3
    }
  }

  /// Set whether display is compact.
  pub fn compact(self, compact: bool) -> HumanDuration {
    HumanDuration {
      compact,
      ..self
    }
  }

  /// Set the number of parts to display (0 for all).
  pub fn parts(self, parts: i32) -> HumanDuration {
    HumanDuration {
      parts,
      ..self
    }
  }
}

/// Make a duration displayable.
pub fn duration<D: Into<HumanDuration>>(dur: D) -> HumanDuration {
  dur.into()
}

/// Create a duration from seconds.
pub fn seconds(secs: f64) -> HumanDuration {
  HumanDuration::new_from_secs(secs)
}

impl fmt::Display for HumanDuration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.seconds.abs() < MIN_SECS {
      return write!(f, "{}", scalar(self.seconds).suffix("s").space(!self.compact));
    }

    let mut pw = PartWriter::new(f, self);

    if pw.keep_going() && self.seconds > WEEK_SECS {
      let weeks = self.seconds / WEEK_SECS;
      pw.put_part(weeks, 0, "w", "weeks")?;
    }

    if pw.keep_going() && self.seconds > DAY_SECS {
      let days = self.seconds % WEEK_SECS / DAY_SECS;
      pw.put_part(days, 0, "d", "days")?;
    }

    if pw.keep_going() && self.seconds > HOUR_SECS {
      let hours = self.seconds % DAY_SECS / HOUR_SECS;
      pw.put_part(hours, 0, "h", "hours")?;
    }

    if pw.keep_going() && self.seconds > MIN_SECS {
      let mins = self.seconds % HOUR_SECS / MIN_SECS;
      pw.put_part(mins, 0, "m", "minutes")?;
    }

    if pw.keep_going() {
      let secs = self.seconds % MIN_SECS;
      pw.put_part(secs, 2, "s", "seconds")?;
    }

    Ok(())
  }
}

struct PartWriter<'a, 'b> {
  fmt: &'a mut fmt::Formatter<'b>,
  parts: i32,
  written: i32,
  compact: bool
}

impl <'a, 'b> PartWriter<'a, 'b> {
  fn new(fmt: &'a mut fmt::Formatter<'b>, dur: &HumanDuration) -> PartWriter<'a, 'b> {
    PartWriter {
      fmt, parts: dur.parts, written: 0, compact: dur.compact
    }
  }

  fn keep_going(&self) -> bool {
    self.parts <= 0 || self.written < self.parts
  }

  fn put_part(&mut self, val: f64, prec: usize, short: &'static str, long: &'static str) -> fmt::Result {
    let v = if prec == 0 {
      val.floor()
    } else {
      val
    };
    if self.written > 0 && !self.compact {
      self.fmt.write_str(" ")?;
    }
    write!(self.fmt, "{:.*}", prec, v)?;
    if self.compact {
      self.fmt.write_str(short)?;
    } else {
      write!(self.fmt, " {}", long)?;
    }
    self.written += 1;
    Ok(())
  }
}

#[test]
fn test_ms() {
  let ms = seconds(0.324);
  assert_eq!(ms.to_string().as_str(), "324.0ms");
}

#[test]
fn test_seconds() {
  let d = seconds(5.29314);
  assert_eq!(d.to_string().as_str(), "5.293s");
}


#[test]
fn test_hms() {
  let d = seconds(5.0 * 3600.0 + 32.0 * 60.0 + 10.5);
  assert_eq!(d.to_string().as_str(), "5h32m10.50s");
}


#[test]
fn test_hm() {
  let d = seconds(5.0 * 3600.0 + 32.0 * 60.0 + 10.5).parts(2);
  assert_eq!(d.to_string().as_str(), "5h32m");
}


#[test]
fn test_hms_full() {
  let d = seconds(5.0 * 3600.0 + 32.0 * 60.0 + 10.5).compact(false);
  assert_eq!(d.to_string().as_str(), "5 hours 32 minutes 10.50 seconds");
}

#[cfg(feature="chrono")]
#[test]
fn test_chrono() {
  let dur = chrono::Duration::seconds(1042) + chrono::Duration::milliseconds(314);
  let d = duration(dur);
  assert_eq!(d.to_string().as_str(), "17m22.31s");
}
