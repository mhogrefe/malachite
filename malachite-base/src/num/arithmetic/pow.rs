use num::arithmetic::traits::{Pow, PowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_pow {
    ($t:ident) => {
        impl Pow<u64> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: u64) -> $t {
                $t::pow(self, u32::exact_from(exp))
            }
        }

        impl PowAssign<u64> for $t {
            /// Replaces `self` with `self` raised to the power of `exp`.
            ///
            /// $x \gets x^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::pow` module.
            #[inline]
            fn pow_assign(&mut self, exp: u64) {
                *self = $t::pow(*self, u32::exact_from(exp));
            }
        }
    };
}
apply_to_primitive_ints!(impl_pow);
