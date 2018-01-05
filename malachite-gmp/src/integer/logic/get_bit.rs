use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

impl Integer {
    /// Determines whether the `index`th bit of an `Integer`, or the coefficient of 2^(`index`) in
    /// its binary expansion, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).get_bit(2), false);
    /// assert_eq!(Integer::from(123).get_bit(3), true);
    /// assert_eq!(Integer::from(123).get_bit(100), false);
    /// assert_eq!(Integer::from(-123).get_bit(0), true);
    /// assert_eq!(Integer::from(-123).get_bit(1), false);
    /// assert_eq!(Integer::from(-123).get_bit(100), true);
    /// assert_eq!(Integer::trillion().get_bit(12), true);
    /// assert_eq!(Integer::trillion().get_bit(100), false);
    /// assert_eq!((-Integer::trillion()).get_bit(12), true);
    /// assert_eq!((-Integer::trillion()).get_bit(100), true);
    /// ```
    pub fn get_bit(&self, index: u64) -> bool {
        match *self {
            Small(small) if small >= 0 => index < 31 && small & (1 << index) != 0,
            Small(small) => index >= 31 || small & (1 << index) != 0,
            Large(ref large) => (unsafe { gmp::mpz_tstbit(large, index) }) != 0,
        }
    }
}
