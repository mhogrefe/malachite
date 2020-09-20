use num::arithmetic::traits::{
    WrappingMul, WrappingSub, WrappingSubAssign, WrappingSubMul, WrappingSubMulAssign,
};

fn _wrapping_sub_mul<T: WrappingMul<T, Output = T> + WrappingSub<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.wrapping_sub(y.wrapping_mul(z))
}

fn _wrapping_sub_mul_assign<T: WrappingMul<T, Output = T> + WrappingSubAssign<T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.wrapping_sub_assign(y.wrapping_mul(z));
}

macro_rules! impl_wrapping_sub_mul {
    ($t:ident) => {
        impl WrappingSubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSubMul;
            ///
            /// assert_eq!(127i8.wrapping_sub_mul(2, 100), -73);
            /// assert_eq!((-127i8).wrapping_sub_mul(2, 100), -71);
            /// ```
            #[inline]
            fn wrapping_sub_mul(self, y: $t, z: $t) -> $t {
                _wrapping_sub_mul(self, y, z)
            }
        }

        impl WrappingSubMulAssign<$t> for $t {
            /// Replaces `self` with `self - y * z`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSubMulAssign;
            ///
            /// let mut x = 127i8;
            /// x.wrapping_sub_mul_assign(2, 100);
            /// assert_eq!(x, -73);
            ///
            /// let mut x = -127i8;
            /// x.wrapping_sub_mul_assign(2, 100);
            /// assert_eq!(x, -71);
            /// ```
            #[inline]
            fn wrapping_sub_mul_assign(&mut self, y: $t, z: $t) {
                _wrapping_sub_mul_assign(self, y, z)
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_sub_mul);
