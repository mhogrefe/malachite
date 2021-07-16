use num::arithmetic::traits::{SaturatingPow, SaturatingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_saturating_pow {
    ($t:ident) => {
        impl SaturatingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn saturating_pow(self, exp: u64) -> $t {
                $t::saturating_pow(self, u32::exact_from(exp))
            }
        }

        impl SaturatingPowAssign<u64> for $t {
            /// Replaces `self` with `self ^ exp`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x^y & m \leq x^y \leq M \\\\
            ///     M & x^y > M \\\\
            ///     m & x^y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_pow` module.
            #[inline]
            fn saturating_pow_assign(&mut self, exp: u64) {
                *self = SaturatingPow::saturating_pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_pow);
