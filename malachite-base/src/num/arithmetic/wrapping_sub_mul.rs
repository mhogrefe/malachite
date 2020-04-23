use num::arithmetic::traits::{WrappingSubAssign, WrappingSubMul, WrappingSubMulAssign};

macro_rules! impl_wrapping_sub_mul {
    ($t:ident) => {
        impl WrappingSubMul for $t {
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
                self.wrapping_sub(y.wrapping_mul(z))
            }
        }

        impl WrappingSubMulAssign for $t {
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
                self.wrapping_sub_assign(y.wrapping_mul(z));
            }
        }
    };
}

impl_wrapping_sub_mul!(u8);
impl_wrapping_sub_mul!(u16);
impl_wrapping_sub_mul!(u32);
impl_wrapping_sub_mul!(u64);
impl_wrapping_sub_mul!(u128);
impl_wrapping_sub_mul!(usize);
impl_wrapping_sub_mul!(i8);
impl_wrapping_sub_mul!(i16);
impl_wrapping_sub_mul!(i32);
impl_wrapping_sub_mul!(i64);
impl_wrapping_sub_mul!(i128);
impl_wrapping_sub_mul!(isize);
