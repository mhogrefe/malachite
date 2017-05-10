use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether `self` is an integer power of 2.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0).is_power_of_two(), false);
    /// assert_eq!(Natural::from(123).is_power_of_two(), false);
    /// assert_eq!(Natural::from(128).is_power_of_two(), true);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().is_power_of_two(), false);
    /// assert_eq!(Natural::from_str("1099511627776").unwrap().is_power_of_two(), true);
    /// ```
    pub fn is_power_of_two(&self) -> bool {
        match *self {
            Small(x) => x != 0 && x & (x - 1) == 0,
            Large(ref xs) => {
                xs.into_iter().take(xs.len() - 1).all(|&x| x == 0) &&
                {
                    let last = xs.last().unwrap();
                    last & (last - 1) == 0
                }
            }
        }
    }
}
