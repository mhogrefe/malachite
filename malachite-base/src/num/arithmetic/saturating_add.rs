use num::arithmetic::traits::{SaturatingAdd, SaturatingAddAssign};

macro_rules! impl_saturating_add {
    ($t:ident) => {
        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_add(self, other: $t) -> $t {
                $t::saturating_add(self, other)
            }
        }

        impl SaturatingAddAssign<$t> for $t {
            /// Replaces `self` with `self + other`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x + y & m \leq x + y \leq M \\\\
            ///     M & x + y > M \\\\
            ///     m & x + y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `$t::MIN` and $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_add` module.
            #[inline]
            fn saturating_add_assign(&mut self, other: $t) {
                *self = self.saturating_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_add);
