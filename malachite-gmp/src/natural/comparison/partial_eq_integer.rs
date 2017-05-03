use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123) == Integer::from(123));
/// assert!(Natural::from(123) != Integer::from(5));
/// ```
impl PartialEq<Integer> for Natural {
    fn eq(&self, i: &Integer) -> bool {
        match self {
            &Natural::Small(x) => {
                match i {
                    &Integer::Small(y) => y >= 0 && x == (y as u32),
                    &Integer::Large(ref y) => (unsafe { gmp::mpz_cmp_si(y, x.into()) }) == 0,
                }
            }
            &Natural::Large(ref x) => {
                match i {
                    &Integer::Small(_) => false,
                    &Integer::Large(ref y) => (unsafe { gmp::mpz_cmp(x, y) }) == 0,
                }
            }
        }
    }
}
