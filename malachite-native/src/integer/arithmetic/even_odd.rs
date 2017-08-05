use integer::Integer;

impl Integer {
    /// Determines whether `self` is even.
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
    /// assert_eq!(Integer::from(0).is_even(), true);
    /// assert_eq!(Integer::from(123).is_even(), false);
    /// assert_eq!(Integer::from(-128).is_even(), true);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().is_even(), true);
    /// assert_eq!(Integer::from_str("-1000000000001").unwrap().is_even(), false);
    /// ```
    pub fn is_even(&self) -> bool {
        match *self {
            Integer { ref abs, .. } => abs.is_even(),
        }
    }

    /// Determines whether `self` is odd.
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
    /// assert_eq!(Integer::from(0).is_odd(), false);
    /// assert_eq!(Integer::from(123).is_odd(), true);
    /// assert_eq!(Integer::from(-128).is_odd(), false);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().is_odd(), false);
    /// assert_eq!(Integer::from_str("-1000000000001").unwrap().is_odd(), true);
    /// ```
    pub fn is_odd(&self) -> bool {
        match *self {
            Integer { ref abs, .. } => abs.is_odd(),
        }
    }
}
