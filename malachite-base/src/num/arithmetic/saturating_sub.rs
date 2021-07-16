use num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};

macro_rules! impl_saturating_sub {
    ($t:ident) => {
        impl SaturatingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_sub(self, other: $t) -> $t {
                $t::saturating_sub(self, other)
            }
        }

        impl SaturatingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - y & m \leq x - y \leq M \\\\
            ///     M & x - y > M \\\\
            ///     m & x - y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_sub` module.
            #[inline]
            fn saturating_sub_assign(&mut self, other: $t) {
                *self = self.saturating_sub(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_sub);
