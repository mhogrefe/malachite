use num::arithmetic::traits::{
    ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

macro_rules! impl_mod_power_of_2_square {
    ($t:ident) => {
        impl ModPowerOf2Square for $t {
            type Output = $t;

            /// Computes `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
            ///
            /// assert_eq!(5u8.mod_power_of_2_square(3), 1);
            /// assert_eq!(100u32.mod_power_of_2_square(8), 16);
            /// ```
            #[inline]
            fn mod_power_of_2_square(self, pow: u64) -> $t {
                self.mod_power_of_2_mul(self, pow)
            }
        }

        impl ModPowerOf2SquareAssign for $t {
            /// Replaces `self` with `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_2_square_assign(3);
            /// assert_eq!(n, 1);
            ///
            /// let mut n = 100u32;
            /// n.mod_power_of_2_square_assign(8);
            /// assert_eq!(n, 16);
            /// ```
            #[inline]
            fn mod_power_of_2_square_assign(&mut self, pow: u64) {
                self.mod_power_of_2_mul_assign(*self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_square);
