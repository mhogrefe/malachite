use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether a `Natural` is an integer power of 2.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.is_power_of_2(), false);
    ///     assert_eq!(Natural::from(123u32).is_power_of_2(), false);
    ///     assert_eq!(Natural::from(0x80u32).is_power_of_2(), true);
    ///     assert_eq!(Natural::trillion().is_power_of_2(), false);
    ///     assert_eq!(Natural::from_str("1099511627776").unwrap().is_power_of_2(), true);
    /// }
    /// ```
    pub fn is_power_of_2(&self) -> bool {
        match *self {
            Small(small) => small != 0 && small & (small - 1) == 0,
            Large(ref limbs) => {
                limbs
                    .into_iter()
                    .take(limbs.len() - 1)
                    .all(|&limb| limb == 0) && {
                    let last = limbs.last().unwrap();
                    last & (last - 1) == 0
                }
            }
        }
    }
}
