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
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingMulAssign;
            ///
            /// let mut x = 123u16;
            /// x.saturating_mul_assign(456);
            /// assert_eq!(x, 56088);
            ///
            /// let mut x = 123u8;
            /// x.saturating_mul_assign(200);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_mul_assign(&mut self, other: $t) {
                *self = self.saturating_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_mul);
