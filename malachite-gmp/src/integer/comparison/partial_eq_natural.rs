use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to a `Natural`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// assert!(Integer::from(123) == Natural::from(123));
/// assert!(Integer::from(123) != Natural::from(5));
/// ```
impl PartialEq<Natural> for Integer {
    fn eq(&self, n: &Natural) -> bool {
        match *self {
            Integer::Small(x) => {
                match *n {
                    Natural::Small(y) => x >= 0 && y == (x as u32),
                    Natural::Large(_) => false,
                }
            }
            Integer::Large(ref x) => {
                match *n {
                    Natural::Small(y) => (unsafe { gmp::mpz_cmp_ui(x, y.into()) }) == 0,
                    Natural::Large(ref y) => (unsafe { gmp::mpz_cmp(x, y) }) == 0,
                }
            }
        }
    }
}
