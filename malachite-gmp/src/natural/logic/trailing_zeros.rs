use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the number of trailing zeros in the binary expansion of `self` (equivalently, the
    /// multiplicity of 2 in the prime factorization of `self`) or `None` is `self` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0u32).trailing_zeros(), None);
    /// assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    /// assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    /// assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Small(0) => None,
            Small(small) => Some(small.trailing_zeros() as u64),
            Large(ref large) => Some(unsafe { gmp::mpz_scan1(large, 0) }),
        }
    }
}
