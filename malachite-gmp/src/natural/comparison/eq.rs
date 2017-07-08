use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

/// Determines whether a `Natural` is equal to another `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123u32) == Natural::from(123u32));
/// assert!(Natural::from(123u32) != Natural::from(5u32));
/// ```
impl PartialEq<Natural> for Natural {
    fn eq(&self, other: &Natural) -> bool {
        match (self, other) {
            (&Small(x), &Small(y)) => x == y,
            (&Large(ref x), &Large(ref y)) => (unsafe { gmp::mpz_cmp(x, y) }) == 0,
            _ => false,
        }
    }
}

/// Asserts that `Natural` equality is an equivalence relation.
impl Eq for Natural {}
