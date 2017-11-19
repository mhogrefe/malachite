use integer::Integer;
use natural::Natural;
use malachite_base::traits::{Assign, Zero};

impl Natural {
    /// Converts a `Natural` to an `Integer`, taking the `Natural` by value.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).into_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().into_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn into_integer(self) -> Integer {
        let mut n = Integer::zero();
        n.assign(self);
        n
    }

    /// Converts a `Natural` to an `Integer`, taking the `Natural` by reference.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).to_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn to_integer(&self) -> Integer {
        let mut n = Integer::zero();
        n.assign(self);
        n
    }
}
