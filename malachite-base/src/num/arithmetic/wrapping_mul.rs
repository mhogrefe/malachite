use num::arithmetic::traits::{WrappingMul, WrappingMulAssign};

macro_rules! impl_wrapping_mul {
    ($t:ident) => {
        impl WrappingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_mul(self, other: $t) -> $t {
                $t::wrapping_mul(self, other)
            }
        }

        impl WrappingMulAssign<$t> for $t {
            /// Replaces `self` with `self * other`, wrapping around at the boundary of the type.
            ///
            /// $x \gets z$, where $z \equiv xy \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_mul` module.
            #[inline]
            fn wrapping_mul_assign(&mut self, other: $t) {
                *self = self.wrapping_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_mul);
