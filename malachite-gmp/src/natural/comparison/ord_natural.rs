use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to a `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert!(Natural::from(123) > Natural::from(122));
/// assert!(Natural::from(123) >= Natural::from(122));
/// assert!(Natural::from(123) < Natural::from(124));
/// assert!(Natural::from(123) <= Natural::from(124));
/// assert!(Natural::from_str("1000000000000").unwrap() > Natural::from(123));
/// assert!(Natural::from_str("1000000000000").unwrap() >= Natural::from(123));
/// ```
impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Asserts that `Natural` ordering is a total order.
impl Ord for Natural {
    fn cmp(&self, other: &Natural) -> Ordering {
        match (self, other) {
            (&Small(ref x), y) => x.partial_cmp(y).unwrap(),
            (&Large(_), &Small(ref y)) => self.partial_cmp(y).unwrap(),
            (&Large(ref x), &Large(ref y)) => (unsafe { gmp::mpz_cmp(x, y) }).cmp(&0),
        }
    }
}
