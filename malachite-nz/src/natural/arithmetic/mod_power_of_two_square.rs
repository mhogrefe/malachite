use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoMul, ModPowerOfTwoMulAssign, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
};

use natural::Natural;

impl ModPowerOfTwoSquare for Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by value. Assumes the input is
    /// already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquare;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_two_square(2), 0);
    /// assert_eq!(Natural::from(5u32).mod_power_of_two_square(3), 1);
    /// assert_eq!(
    ///     Natural::from_str("12345678987654321").unwrap().mod_power_of_two_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_square(mut self, pow: u64) -> Natural {
        self.mod_power_of_two_square_assign(pow);
        self
    }
}

impl<'a> ModPowerOfTwoSquare for &'a Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by reference. Assumes the
    /// input is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquare;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_two_square(2), 0);
    /// assert_eq!((&Natural::from(5u32)).mod_power_of_two_square(3), 1);
    /// assert_eq!(
    ///     (&Natural::from_str("12345678987654321").unwrap())
    ///         .mod_power_of_two_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_square(self, pow: u64) -> Natural {
        self.mod_power_of_two_mul(self, pow)
    }
}

impl ModPowerOfTwoSquareAssign for Natural {
    /// Replaces `self` with `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is already
    /// reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_two_square_assign(2);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(5u32);
    /// n.mod_power_of_two_square_assign(3);
    /// assert_eq!(n, 1);
    ///
    /// let mut n = Natural::from_str("12345678987654321").unwrap();
    /// n.mod_power_of_two_square_assign(64);
    /// assert_eq!(n.to_string(), "16556040056090124897");
    /// ```
    #[inline]
    fn mod_power_of_two_square_assign(&mut self, pow: u64) {
        self.mod_power_of_two_mul_assign(self.clone(), pow);
    }
}
