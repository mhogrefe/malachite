use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns whether `self` is divisible by 2^(`pow`). If `self` is 0, the result is always true;
    /// otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but more efficient.
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
    ///     assert_eq!(Natural::ZERO.divisible_by_power_of_2(100), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by_power_of_2(2), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by_power_of_2(3), false);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().divisible_by_power_of_2(12),
    ///         true);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().divisible_by_power_of_2(13),
    ///         false);
    /// }
    /// ```
    pub fn divisible_by_power_of_2(&self, pow: u32) -> bool {
        match (self, pow) {
            (_, 0) => true,
            (&Small(0), _) => true,
            (&Small(_), pow) if pow >= 32 => false,
            (&Small(small), pow) => small & ((1 << pow) - 1) == 0,
            (&Large(ref large), pow) => unsafe {
                gmp::mpz_divisible_2exp_p(large, pow.into()) != 0
            },
        }
    }
}
