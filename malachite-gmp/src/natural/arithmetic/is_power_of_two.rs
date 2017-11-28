use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether a `Natural` is an integer power of 2.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.is_power_of_two(), false);
    ///     assert_eq!(Natural::from(123u32).is_power_of_two(), false);
    ///     assert_eq!(Natural::from(128u32).is_power_of_two(), true);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().is_power_of_two(), false);
    ///     assert_eq!(Natural::from_str("1099511627776").unwrap().is_power_of_two(), true);
    /// }
    /// ```
    pub fn is_power_of_two(&self) -> bool {
        match *self {
            Small(small) => small != 0 && small & (small - 1) == 0,
            Large(ref large) => unsafe {
                gmp::mpz_scan1(large, 0) == (gmp::mpz_sizeinbase(large, 2)) as u64 - 1
            },
        }
    }
}
