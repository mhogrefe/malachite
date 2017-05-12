use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to a `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// assert!(Integer::from(123) == Natural::from(123));
/// assert!(Integer::from(123) != Natural::from(5));
/// ```
impl PartialEq<Natural> for Integer {
    fn eq(&self, n: &Natural) -> bool {
        match (self, n) {
            (&Integer::Small(x), &Natural::Small(y)) => x >= 0 && y == (x as u32),
            (&Integer::Small(_), &Natural::Large(_)) => false,
            (&Integer::Large(ref x), &Natural::Small(y)) => {
                (unsafe { gmp::mpz_cmp_ui(x, y.into()) }) == 0
            }
            (&Integer::Large(ref x), &Natural::Large(ref y)) => {
                (unsafe { gmp::mpz_cmp(x, y) }) == 0
            }
        }
    }
}
