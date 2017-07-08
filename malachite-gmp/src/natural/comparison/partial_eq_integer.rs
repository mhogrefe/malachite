use gmp_mpfr_sys::gmp;
use integer::Integer;
use natural::Natural;

/// Determines whether a `Natural` is equal to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123u32) == Integer::from(123));
/// assert!(Natural::from(123u32) != Integer::from(5));
/// ```
impl PartialEq<Integer> for Natural {
    fn eq(&self, other: &Integer) -> bool {
        match (self, other) {
            (&Natural::Small(x), &Integer::Small(y)) => y >= 0 && x == (y as u32),
            (&Natural::Small(x), &Integer::Large(ref y)) => {
                (unsafe { gmp::mpz_cmp_si(y, x.into()) }) == 0
            }
            (&Natural::Large(_), &Integer::Small(_)) => false,
            (&Natural::Large(ref x), &Integer::Large(ref y)) => {
                (unsafe { gmp::mpz_cmp(x, y) }) == 0
            }
        }
    }
}
