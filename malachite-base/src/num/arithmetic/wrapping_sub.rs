use num::arithmetic::traits::{WrappingSub, WrappingSubAssign};

macro_rules! impl_wrapping_sub {
    ($t:ident) => {
        impl WrappingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, other: $t) -> $t {
                $t::wrapping_sub(self, other)
            }
        }

        impl WrappingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`, wrapping around at the boundary of the type.
            ///
            /// $x \gets z$, where $z \equiv x - y \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_sub` module.
            #[inline]
            fn wrapping_sub_assign(&mut self, other: $t) {
                *self = self.wrapping_sub(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_sub);
