use num::arithmetic::traits::{OverflowingMul, OverflowingMulAssign};

macro_rules! impl_overflowing_mul {
    ($t:ident) => {
        impl OverflowingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_mul(self, other: $t) -> ($t, bool) {
                $t::overflowing_mul(self, other)
            }
        }

        impl OverflowingMulAssign<$t> for $t {
            /// Replaces `self` with `self * other`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_mul` module.
            #[inline]
            fn overflowing_mul_assign(&mut self, other: $t) -> bool {
                let (result, overflow) = self.overflowing_mul(other);
                *self = result;
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_mul);
