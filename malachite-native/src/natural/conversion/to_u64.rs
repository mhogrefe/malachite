use natural::make_u64;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Converts a `Natural` to a `u64`, returning `None` if the `Natural` is too large.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(123u32).to_u64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Natural::from_str("1000000000000000000000").unwrap().to_u64()),
    ///            "None");
    /// ```
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            Small(small) => Some(small.into()),
            Large(ref limbs) if limbs.len() == 2 => Some(make_u64(limbs[1], limbs[0])),
            Large(_) => None,
        }
    }

    /// Converts a `Natural` to a `u64`, wrapping mod 2^(64).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).to_u64_wrapping().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000000000000")
    ///                 .unwrap().to_u64_wrapping().to_string(),
    ///            "3875820019684212736");
    /// ```
    pub fn to_u64_wrapping(&self) -> u64 {
        match *self {
            Small(small) => small.into(),
            Large(ref limbs) => make_u64(limbs[1], limbs[0]),
        }
    }
}
