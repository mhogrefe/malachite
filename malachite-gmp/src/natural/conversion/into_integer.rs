use integer::Integer;
use natural::Natural;
use traits::Assign;

impl Natural {
    /// Converts a `Natural` to an `Integer`, consuming the `Natural`.
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
        let mut i = Integer::new();
        i.assign(&self);
        i
    }
}
