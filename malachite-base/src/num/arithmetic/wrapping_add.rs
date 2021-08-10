use num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};

macro_rules! impl_wrapping_add {
    ($t:ident) => {
        impl WrappingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_add(self, other: $t) -> $t {
                $t::wrapping_add(self, other)
            }
        }

        impl WrappingAddAssign<$t> for $t {
            /// Replaces `self` with `self + other`, wrapping around at the boundary of the type.
            ///
            /// $x \gets z$, where $z \equiv x + y \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_add` module.
            #[inline]
            fn wrapping_add_assign(&mut self, other: $t) {
                *self = self.wrapping_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add);
