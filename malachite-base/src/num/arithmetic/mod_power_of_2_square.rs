use num::arithmetic::traits::{
    ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

macro_rules! impl_mod_power_of_2_square {
    ($t:ident) => {
        impl ModPowerOf2Square for $t {
            type Output = $t;

            /// Computes `self.square()` mod $2^p$. Assumes the input is already reduced mod $2^p$.
            ///
            /// $f(x, p) = y$, where $x, y < 2^p$ and $x^2 \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_square` module.
            #[inline]
            fn mod_power_of_2_square(self, pow: u64) -> $t {
                self.mod_power_of_2_mul(self, pow)
            }
        }

        impl ModPowerOf2SquareAssign for $t {
            /// Replaces `self` with `self.square()` mod $2^p$. Assumes the input is already
            /// reduced mod $2^p$.
            ///
            /// $x \gets y$, where $x, y < 2^p$ and $x^2 \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_square` module.
            #[inline]
            fn mod_power_of_2_square_assign(&mut self, pow: u64) {
                self.mod_power_of_2_mul_assign(*self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_square);
