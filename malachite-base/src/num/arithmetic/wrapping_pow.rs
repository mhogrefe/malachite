use num::arithmetic::traits::{WrappingPow, WrappingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_wrapping_pow {
    ($t:ident) => {
        impl WrappingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_pow(self, exp: u64) -> $t {
                $t::wrapping_pow(self, u32::exact_from(exp))
            }
        }

        impl WrappingPowAssign<u64> for $t {
            /// Replaces `self` with `self` raised to the power of `exp`, wrapping around at the
            /// boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv x^n \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_pow` module.
            #[inline]
            fn wrapping_pow_assign(&mut self, exp: u64) {
                *self = WrappingPow::wrapping_pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_pow);
