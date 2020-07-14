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
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingMulAssign;
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_mul_assign(456), false);
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// assert_eq!(x.overflowing_mul_assign(200), true);
            /// assert_eq!(x, 24);
            /// ```
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
