use integer::Integer;
use natural::Natural;

impl Natural {
    /// Converts a `Natural` to an `Integer`. This implementation takes `self` by value.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).into_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().into_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn into_integer(self) -> Integer {
        Integer {
            sign: true,
            abs: self,
        }
    }

    /// Converts a `Natural` to an `Integer`. This implementation takes `self` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).to_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn to_integer(&self) -> Integer {
        Integer {
            sign: true,
            abs: self.clone(),
        }
    }
}
