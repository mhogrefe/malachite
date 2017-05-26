use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;

/// Compares `self` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123u32) > Integer::from(122));
/// assert!(Natural::from(123u32) >= Integer::from(122));
/// assert!(Natural::from(123u32) < Integer::from(124));
/// assert!(Natural::from(123u32) <= Integer::from(124));
/// assert!(Natural::from(123u32) > Integer::from(-123));
/// assert!(Natural::from(123u32) >= Integer::from(-123));
/// ```
impl PartialOrd<Integer> for Natural {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        match (self, other) {
            (&Natural::Small(ref x), y) => x.partial_cmp(y),
            (&Natural::Large(_), &Integer::Small(y)) if y < 0 => Some(Ordering::Greater),
            (&Natural::Large(_), &Integer::Small(y)) => self.partial_cmp(&(y as u32)),
            (&Natural::Large(ref x), &Integer::Large(ref y)) => {
                (unsafe { gmp::mpz_cmp(x, y) }).partial_cmp(&0)
            }
        }
    }
}
