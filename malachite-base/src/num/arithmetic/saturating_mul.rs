use num::arithmetic::traits::{SaturatingMul, SaturatingMulAssign};

macro_rules! impl_saturating_mul {
    ($t:ident) => {
        impl SaturatingMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_mul(self, rhs: $t) -> $t {
                $t::saturating_mul(self, rhs)
            }
        }

        impl SaturatingMulAssign for $t {
            /// Replaces `self` with `self * rhs`, saturating at the numeric bounds instead of
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
            /// assert_eq!(x, 56_088);
            ///
            /// let mut x = 123u8;
            /// x.saturating_mul_assign(200);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_mul_assign(&mut self, rhs: $t) {
                *self = self.saturating_mul(rhs);
            }
        }
    };
}

impl_saturating_mul!(u8);
impl_saturating_mul!(u16);
impl_saturating_mul!(u32);
impl_saturating_mul!(u64);
impl_saturating_mul!(u128);
impl_saturating_mul!(usize);
impl_saturating_mul!(i8);
impl_saturating_mul!(i16);
impl_saturating_mul!(i32);
impl_saturating_mul!(i64);
impl_saturating_mul!(i128);
impl_saturating_mul!(isize);
