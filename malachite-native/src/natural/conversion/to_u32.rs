use natural::Natural::{self, Large, Small};

impl Natural {
    /// Converts a `Natural` to a `u32`, returning `None` if the `Natural` is too large.
    ///
    /// Time: worst case O(1)
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(123).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Natural::from_str("1000000000000").unwrap().to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            Small(small) => Some(small),
            Large(_) => None,
        }
    }

    /// Converts a `Natural` to a `u32`, wrapping mod 2^(32).
    ///
    /// Time: worst case O(1)
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "3567587328");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Small(small) => small,
            Large(ref limbs) => limbs[0],
        }
    }
}
