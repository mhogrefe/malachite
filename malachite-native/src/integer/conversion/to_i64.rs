use integer::Integer;
use natural::Natural;

impl Integer {
    /// Converts an `Integer` to an `i64`, returning `None` if the `Integer` is outside the range of
    /// an `i64`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_i64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_i64()), "Some(-123)");
    /// assert_eq!(
    ///     format!("{:?}", Integer::from_str("1000000000000000000000000").unwrap().to_i64()),
    ///     "None");
    /// assert_eq!(format!("{:?}",
    ///     Integer::from_str("-1000000000000000000000000").unwrap().to_i64()),
    ///     "None");
    /// ```
    pub fn to_i64(&self) -> Option<i64> {
        if self.significant_bits() < 64 || *self == -((Natural::from(1u32) << 63).into_integer()) {
            Some(self.to_i64_wrapping())
        } else {
            None
        }
    }

    /// Converts an `Integer` to a `i64`, wrapping mod 2^(64).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).to_i64_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_i64_wrapping().to_string(), "-123");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000000000000000").unwrap().to_i64_wrapping().to_string(),
    ///     "2003764205206896640");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000").unwrap().to_i64_wrapping().to_string(),
    ///     "-2003764205206896640");
    /// ```
    pub fn to_i64_wrapping(&self) -> i64 {
        self.to_u64_wrapping() as i64
    }
}
