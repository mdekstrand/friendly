//! Routines for significant figures
use std::cmp::max;

/// Adjust a value for the specified number of significant figures.
///
/// Returns a tuple of the adjusted value (rounded as appropriate) and the number of
/// values after the decimal point needed to display it.
pub fn sigscale(val: f64, sf: usize) -> (f64, usize) {
    if val.is_normal() {
        let sf = sf as i32;
        let log = val.abs().log10();
        let mut scale = log.ceil() as i32;
        if log == log.ceil() {
            scale = scale + 1; // we're exactly at the boundary
        }
        // how much do we need to shift befeore rounding?
        let scale_diff = sf - scale;
        let adj = 10.0f64.powi(scale_diff);
        let adj_val = (val * adj).round() / adj;
        (adj_val, max(scale_diff, 0) as usize)
    } else {
        (val, sf)
    }
}

#[test]
fn test_zero() {
    let (v, prec) = sigscale(0.0, 4);
    assert_eq!(v, 0.0);
    assert_eq!(prec, 4);
}

#[test]
fn test_one() {
    let (v, prec) = sigscale(1.0, 4);
    assert_eq!(v, 1.0);
    assert_eq!(prec, 3);
}

#[test]
fn test_two_digits() {
    let (v, prec) = sigscale(15.3234, 4);
    assert_eq!(v, 15.32);
    assert_eq!(prec, 2);
}

#[test]
fn test_small() {
    let (v, prec) = sigscale(0.000315772, 2);
    assert_eq!(v, 0.00032);
    assert_eq!(prec, 5);
}
