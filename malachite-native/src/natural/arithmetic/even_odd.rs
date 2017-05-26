use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether `self` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0u32).is_even(), true);
    /// assert_eq!(Natural::from(123u32).is_even(), false);
    /// assert_eq!(Natural::from(128u32).is_even(), true);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().is_even(), true);
    /// assert_eq!(Natural::from_str("1000000000001").unwrap().is_even(), false);
    /// ```
    pub fn is_even(&self) -> bool {
        match *self {
            Small(small) => small & 1 == 0,
            Large(ref limbs) => limbs[0] & 1 == 0,
        }
    }

    /// Determines whether `self` is odd.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0u32).is_odd(), false);
    /// assert_eq!(Natural::from(123u32).is_odd(), true);
    /// assert_eq!(Natural::from(128u32).is_odd(), false);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().is_odd(), false);
    /// assert_eq!(Natural::from_str("1000000000001").unwrap().is_odd(), true);
    /// ```
    pub fn is_odd(&self) -> bool {
        match *self {
            Small(small) => small & 1 != 0,
            Large(ref limbs) => limbs[0] & 1 != 0,
        }
    }
}
