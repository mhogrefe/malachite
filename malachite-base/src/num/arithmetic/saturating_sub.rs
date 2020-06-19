use num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};

macro_rules! impl_saturating_sub {
    ($t:ident) => {
        impl SaturatingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_sub(self, other: $t) -> $t {
                $t::saturating_sub(self, other)
            }
        }

        impl SaturatingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
            ///
            /// let mut x = 456u16;
            /// x.saturating_sub_assign(123);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// x.saturating_sub_assign(456);
            /// assert_eq!(x, 0);
            /// ```
            #[inline]
            fn saturating_sub_assign(&mut self, other: $t) {
                *self = self.saturating_sub(other);
            }
        }
    };
}

impl_saturating_sub!(u8);
impl_saturating_sub!(u16);
impl_saturating_sub!(u32);
impl_saturating_sub!(u64);
impl_saturating_sub!(u128);
impl_saturating_sub!(usize);
impl_saturating_sub!(i8);
impl_saturating_sub!(i16);
impl_saturating_sub!(i32);
impl_saturating_sub!(i64);
impl_saturating_sub!(i128);
impl_saturating_sub!(isize);
