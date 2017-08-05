use integer::Integer;

impl Integer {
    /// Converts an `Integer` to a `u64`, returning `None` if the `Integer` is negative or too
    /// large.
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
    /// assert_eq!(format!("{:?}", Integer::from(123).to_u64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_u64()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000000000000").unwrap().to_u64()),
    ///                            "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000000000000").unwrap().to_u64()),
    ///                            "None");
    /// ```
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => abs.to_u64(),
        }
    }

    /// Converts an `Integer` to a `u64`, wrapping mod 2^(64).
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
    /// assert_eq!(Integer::from(123).to_u64_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_u64_wrapping().to_string(), "18446744073709551493");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000000000000").unwrap().to_u64_wrapping().to_string(),
    ///     "3875820019684212736");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000").unwrap().to_u64_wrapping().to_string(),
    ///     "14570924054025338880");
    /// ```
    pub fn to_u64_wrapping(&self) -> u64 {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => abs.to_u64_wrapping(),
            Integer {
                sign: false,
                ref abs,
            } => abs.to_u64_wrapping().wrapping_neg(),
        }
    }
}
