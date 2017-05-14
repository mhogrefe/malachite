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
            Small(small) => small != 0 && small & (small - 1) == 0,
            Large(ref limbs) => {
                limbs.into_iter().take(limbs.len() - 1).all(|&limb| limb == 0) &&
                {
                    let last = limbs.last().unwrap();
                    last & (last - 1) == 0
                }
            }
        }
    }
}
