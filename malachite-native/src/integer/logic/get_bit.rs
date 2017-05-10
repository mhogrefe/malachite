use integer::Integer;

impl Integer {
    /// Determines whether the `index`th bit of `self`, or the coefficient of 2^(`index`) in the
    /// binary expansion of `self`, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).get_bit(2), false);
    /// assert_eq!(Integer::from(123).get_bit(3), true);
    /// assert_eq!(Integer::from(123).get_bit(100), false);
    /// assert_eq!(Integer::from(-123).get_bit(0), true);
    /// assert_eq!(Integer::from(-123).get_bit(1), false);
    /// assert_eq!(Integer::from(-123).get_bit(100), true);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().get_bit(100), false);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().get_bit(100), true);
    /// ```
    pub fn get_bit(&self, index: u64) -> bool {
        match *self {
            Integer { sign: true, ref abs } => abs.get_bit(index),
            Integer { sign: false, ref abs } => abs.get_bit_neg(index),
        }
    }
}
