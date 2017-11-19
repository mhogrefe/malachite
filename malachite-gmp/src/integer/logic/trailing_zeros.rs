use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

impl Integer {
    /// Returns the number of trailing zeros in the binary expansion of an `Integer` (equivalently,
    /// the multiplicity of 2 in its prime factorization) or `None` is the `Integer` is 0.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::zero().trailing_zeros(), None);
    ///     assert_eq!(Integer::from(3).trailing_zeros(), Some(0));
    ///     assert_eq!(Integer::from(-72).trailing_zeros(), Some(3));
    ///     assert_eq!(Integer::from(100).trailing_zeros(), Some(2));
    ///     assert_eq!(Integer::from_str("-1000000000000").unwrap().trailing_zeros(), Some(12));
    /// }
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Small(0) => None,
            Small(small) => Some(small.trailing_zeros() as u64),
            Large(ref large) => Some(unsafe { gmp::mpz_scan1(large, 0) }),
        }
    }
}
