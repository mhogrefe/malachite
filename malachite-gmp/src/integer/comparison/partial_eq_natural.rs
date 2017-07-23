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
/// assert!(Integer::from(123) == Natural::from(123u32));
/// assert!(Integer::from(123) != Natural::from(5u32));
/// ```
impl PartialEq<Natural> for Integer {
    fn eq(&self, other: &Natural) -> bool {
        match (self, other) {
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
