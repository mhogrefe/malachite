use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Compares an `Integer` to another `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(-123) < Integer::from(-122));
/// assert!(Integer::from(-123) <= Integer::from(-122));
/// assert!(Integer::from(-123) > Integer::from(-124));
/// assert!(Integer::from(-123) >= Integer::from(-124));
/// ```
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Asserts that `Integer` ordering is a total order.
impl Ord for Integer {
    fn cmp(&self, other: &Integer) -> Ordering {
        if self as *const Integer == other as *const Integer {
            return Ordering::Equal;
        }
        match (self, other) {
            (&Small(ref x), y) => x.partial_cmp(y).unwrap(),
            (&Large(_), &Small(ref y)) => self.partial_cmp(y).unwrap(),
            (&Large(ref x), &Large(ref y)) => (unsafe { gmp::mpz_cmp(x, y) }).cmp(&0),
        }
    }
}
