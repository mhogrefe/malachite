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
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingMulAssign;
            ///
            /// let mut x = 123u16;
            /// x.wrapping_mul_assign(456);
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// x.wrapping_mul_assign(200);
            /// assert_eq!(x, 24);
            /// ```
            #[inline]
            fn wrapping_mul_assign(&mut self, other: $t) {
                *self = self.wrapping_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_mul);
