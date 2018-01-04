use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::i32;

impl Integer {
    /// Returns whether `self` is divisible by 2^(`pow`). If `self` is 0, the result is always true;
    /// otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but more efficient.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.divisible_by_power_of_2(100), true);
    ///     assert_eq!(Integer::from(-100).divisible_by_power_of_2(2), true);
    ///     assert_eq!(Integer::from(100u32).divisible_by_power_of_2(3), false);
    ///     assert_eq!((-Integer::trillion()).divisible_by_power_of_2(12), true);
    ///     assert_eq!(Integer::trillion().divisible_by_power_of_2(13), false);
    /// }
    /// ```
    pub fn divisible_by_power_of_2(&self, pow: u32) -> bool {
        match (self, pow) {
            (_, 0) | (&Small(0), _) => true,
            (&Small(i32::MIN), pow) if pow < 32 => true,
            (&Small(_), pow) if pow >= 31 => false,
            (&Small(small), pow) => small & ((1 << pow) - 1) == 0,
            (&Large(ref large), pow) => unsafe {
                gmp::mpz_divisible_2exp_p(large, pow.into()) != 0
            },
        }
    }
}
