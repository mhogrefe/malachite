use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

impl Integer {
    /// Returns the smallest number of bits necessary to represent the absolute value of an
    /// `Integer`. 0 has zero significant bits.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::zero().significant_bits(), 0);
    ///     assert_eq!(Integer::from(100).significant_bits(), 7);
    ///     assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// }
    /// ```
    pub fn significant_bits(&self) -> u64 {
        match *self {
            Small(small) => (32 - (small.wrapping_abs() as u32).leading_zeros()) as u64,
            Large(ref large) => (unsafe { gmp::mpz_sizeinbase(large, 2) }) as u64,
        }
    }
}
