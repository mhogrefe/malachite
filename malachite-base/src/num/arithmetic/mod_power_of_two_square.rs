use num::arithmetic::traits::{
    ModPowerOfTwoMul, ModPowerOfTwoMulAssign, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
};

macro_rules! impl_mod_power_of_two_square {
    ($t:ident) => {
        impl ModPowerOfTwoSquare for $t {
            type Output = $t;

            /// Computes `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquare;
            ///
            /// assert_eq!(5u8.mod_power_of_two_square(3), 1);
            /// assert_eq!(100u32.mod_power_of_two_square(8), 16);
            /// ```
            #[inline]
            fn mod_power_of_two_square(self, pow: u64) -> $t {
                self.mod_power_of_two_mul(self, pow)
            }
        }

        impl ModPowerOfTwoSquareAssign for $t {
            /// Replaces `self` with `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquareAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_two_square_assign(3);
            /// assert_eq!(n, 1);
            ///
            /// let mut n = 100u32;
            /// n.mod_power_of_two_square_assign(8);
            /// assert_eq!(n, 16);
            /// ```
            #[inline]
            fn mod_power_of_two_square_assign(&mut self, pow: u64) {
                self.mod_power_of_two_mul_assign(*self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_square);
