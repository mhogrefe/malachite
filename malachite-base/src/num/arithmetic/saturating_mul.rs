use num::arithmetic::traits::{SaturatingMul, SaturatingMulAssign};

macro_rules! impl_saturating_mul {
    ($t:ident) => {
        impl SaturatingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_mul(self, other: $t) -> $t {
                $t::saturating_mul(self, other)
            }
        }

        impl SaturatingMulAssign<$t> for $t {
            /// Replaces `self` with `self * other`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     xy & m \leq xy \leq M \\\\
            ///     M & xy > M \\\\
            ///     m & xy < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_mul` module.
            #[inline]
            fn saturating_mul_assign(&mut self, other: $t) {
                *self = self.saturating_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_mul);
