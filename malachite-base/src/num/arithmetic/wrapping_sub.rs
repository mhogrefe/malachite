use num::arithmetic::traits::{WrappingSub, WrappingSubAssign};

macro_rules! impl_wrapping_sub {
    ($t:ident) => {
        impl WrappingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, rhs: $t) -> $t {
                $t::wrapping_sub(self, rhs)
            }
        }

        impl WrappingSubAssign for $t {
            /// Replaces `self` with `self - rhs`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSubAssign;
            ///
            /// let mut x = 456u16;
            /// x.wrapping_sub_assign(123);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// x.wrapping_sub_assign(456);
            /// assert_eq!(x, 65_203);
            /// ```
            #[inline]
            fn wrapping_sub_assign(&mut self, rhs: $t) {
                *self = self.wrapping_sub(rhs);
            }
        }
    };
}

impl_wrapping_sub!(u8);
impl_wrapping_sub!(u16);
impl_wrapping_sub!(u32);
impl_wrapping_sub!(u64);
impl_wrapping_sub!(u128);
impl_wrapping_sub!(usize);
impl_wrapping_sub!(i8);
impl_wrapping_sub!(i16);
impl_wrapping_sub!(i32);
impl_wrapping_sub!(i64);
impl_wrapping_sub!(i128);
impl_wrapping_sub!(isize);
