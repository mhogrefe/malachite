use num::arithmetic::traits::{SubMul, SubMulAssign, WrappingSubMul, WrappingSubMulAssign};

macro_rules! impl_sub_mul {
    ($t:ident) => {
        impl SubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SubMul;
            ///
            /// assert_eq!(60u32.sub_mul(5, 10), 10);
            /// assert_eq!(127i8.sub_mul(2, 100), -73);
            /// ```
            #[inline]
            fn sub_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_sub_mul(y, z)
            }
        }

        impl SubMulAssign<$t> for $t {
            /// Replaces `self` with `self - y * z`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SubMulAssign;
            ///
            /// let mut x = 60u32;
            /// x.sub_mul_assign(5, 10);
            /// assert_eq!(x, 10);
            ///
            /// let mut x = 127i8;
            /// x.sub_mul_assign(2, 100);
            /// assert_eq!(x, -73);
            /// ```
            #[inline]
            fn sub_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_sub_mul_assign(y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_sub_mul);
