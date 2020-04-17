use num::arithmetic::traits::{WrappingMul, WrappingMulAssign};

macro_rules! impl_wrapping_mul {
    ($t:ident) => {
        impl WrappingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_mul(self, rhs: $t) -> $t {
                $t::wrapping_mul(self, rhs)
            }
        }

        impl WrappingMulAssign for $t {
            /// Replaces `self` with `self * rhs`, wrapping around at the boundary of the type.
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
            fn wrapping_mul_assign(&mut self, rhs: $t) {
                *self = self.wrapping_mul(rhs);
            }
        }
    };
}

impl_wrapping_mul!(u8);
impl_wrapping_mul!(u16);
impl_wrapping_mul!(u32);
impl_wrapping_mul!(u64);
impl_wrapping_mul!(u128);
impl_wrapping_mul!(usize);
impl_wrapping_mul!(i8);
impl_wrapping_mul!(i16);
impl_wrapping_mul!(i32);
impl_wrapping_mul!(i64);
impl_wrapping_mul!(i128);
impl_wrapping_mul!(isize);
