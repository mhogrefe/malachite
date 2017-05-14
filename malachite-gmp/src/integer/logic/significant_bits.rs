use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

impl Integer {
    /// Returns the smallest number of bits necessary to represent the absolute value of `self`. 0
    /// has zero significant bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).significant_bits(), 0);
    /// assert_eq!(Integer::from(100).significant_bits(), 7);
    /// assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// ```
    pub fn significant_bits(&self) -> u64 {
        match *self {
            Small(small) => (32 - (small.abs() as u32).leading_zeros()) as u64,
            Large(ref large) => (unsafe { gmp::mpz_sizeinbase(large, 2) }) as u64,
        }
    }
}
