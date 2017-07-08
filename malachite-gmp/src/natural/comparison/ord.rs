use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to another `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123u32) > Natural::from(122u32));
/// assert!(Natural::from(123u32) >= Natural::from(122u32));
/// assert!(Natural::from(123u32) < Natural::from(124u32));
/// assert!(Natural::from(123u32) <= Natural::from(124u32));
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
            (&Large(_), &Small(_)) => Ordering::Greater,
            (&Large(ref x), &Large(ref y)) => (unsafe { gmp::mpz_cmp(x, y) }).cmp(&0),
        }
    }
}
