use integer::Integer;

impl Integer {
    /// Converts an `Integer` to a `u32`, returning `None` if the `Integer` is too large.
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_u32()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000").unwrap().to_u32()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000").unwrap().to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            Integer { sign: false, .. } => None,
            Integer { sign: true, ref abs } => abs.to_u32(),
        }
    }

    /// Converts an `Integer` to a `u32`, wrapping mod 2^(32).
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_u32_wrapping().to_string(), "4294967173");
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "3567587328");
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "727379968");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Integer { sign: true, ref abs } => abs.to_u32_wrapping(),
            Integer { sign: false, ref abs } => abs.to_u32_wrapping().wrapping_neg(),
        }
    }
}
