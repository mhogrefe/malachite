use num::arithmetic::traits::{WrappingNeg, WrappingNegAssign};

macro_rules! impl_wrapping_neg {
    ($t:ident) => {
        impl WrappingNeg for $t {
            type Output = $t;

            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingNegAssign for $t {
            /// Replaces `self` with its negative, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingNegAssign;
            ///
            /// let mut x = 0i8;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100u64;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 18446744073709551516);
            ///
            /// let mut x = -100i64;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.wrapping_neg_assign();
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }
    };
}

impl_wrapping_neg!(u8);
impl_wrapping_neg!(u16);
impl_wrapping_neg!(u32);
impl_wrapping_neg!(u64);
impl_wrapping_neg!(u128);
impl_wrapping_neg!(usize);
impl_wrapping_neg!(i8);
impl_wrapping_neg!(i16);
impl_wrapping_neg!(i32);
impl_wrapping_neg!(i64);
impl_wrapping_neg!(i128);
impl_wrapping_neg!(isize);
