use num::arithmetic::traits::{SaturatingAdd, SaturatingAddAssign};

macro_rules! impl_saturating_add {
    ($t:ident) => {
        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_add` functions in the standard library, for
            /// example [this one](i32::saturating_add).
            #[inline]
            fn saturating_add(self, other: $t) -> $t {
                $t::saturating_add(self, other)
            }
        }

        impl SaturatingAddAssign<$t> for $t {
            /// Adds a number to another number, in place, saturating at the numeric bounds instead
            /// of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x + y & \text{if} \\quad m \leq x + y \leq M, \\\\
            ///     M & \text{if} \\quad x + y > M, \\\\
            ///     m & \text{if} \\quad x + y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_add#saturating_add_assign).
            #[inline]
            fn saturating_add_assign(&mut self, other: $t) {
                *self = self.saturating_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_add);
