use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the number of trailing zeros in the binary expansion of a `Natural` (equivalently,
    /// the multiplicity of 2 in its prime factorization) or `None` is the `Natural` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.trailing_zeros(), None);
    ///     assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    ///     assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    ///     assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    ///     assert_eq!(Natural::trillion().trailing_zeros(), Some(12));
    /// }
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Small(0) => None,
            Small(small) => Some(small.trailing_zeros().into()),
            Large(ref large) => Some(unsafe { gmp::mpz_scan1(large, 0) }),
        }
    }
}
