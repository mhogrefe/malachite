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
/// assert!(Integer::from(123) > Natural::from(122u32));
/// assert!(Integer::from(123) >= Natural::from(122u32));
/// assert!(Integer::from(123) < Natural::from(124u32));
/// assert!(Integer::from(123) <= Natural::from(124u32));
/// assert!(Integer::from(-123) < Natural::from(123u32));
/// assert!(Integer::from(-123) <= Natural::from(123u32));
/// ```
impl PartialOrd<Natural> for Integer {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        match (self, other) {
            (&Integer::Small(x), _) if x < 0 => Some(Ordering::Less),
            (&Integer::Small(x), y) => (x as u32).partial_cmp(y),
            (&Integer::Large(_), &Natural::Small(ref y)) => self.partial_cmp(y),
            (&Integer::Large(ref x), &Natural::Large(ref y)) => {
                (unsafe { gmp::mpz_cmp(x, y) }).partial_cmp(&0)
            }
        }
    }
}
