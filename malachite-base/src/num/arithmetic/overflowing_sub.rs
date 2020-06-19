use num::arithmetic::traits::{OverflowingSub, OverflowingSubAssign};

macro_rules! impl_overflowing_sub {
    ($t:ident) => {
        impl OverflowingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_sub(self, other: $t) -> ($t, bool) {
                $t::overflowing_sub(self, other)
            }
        }

        impl OverflowingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSubAssign;
            ///
            /// let mut x = 456u16;
            /// assert_eq!(x.overflowing_sub_assign(123), false);
            /// assert_eq!(x, 333);
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_sub_assign(456), true);
            /// assert_eq!(x, 65_203);
            /// ```
            #[inline]
            fn overflowing_sub_assign(&mut self, other: $t) -> bool {
                let (result, overflow) = self.overflowing_sub(other);
                *self = result;
                overflow
            }
        }
    };
}

impl_overflowing_sub!(u8);
impl_overflowing_sub!(u16);
impl_overflowing_sub!(u32);
impl_overflowing_sub!(u64);
impl_overflowing_sub!(u128);
impl_overflowing_sub!(usize);
impl_overflowing_sub!(i8);
impl_overflowing_sub!(i16);
impl_overflowing_sub!(i32);
impl_overflowing_sub!(i64);
impl_overflowing_sub!(i128);
impl_overflowing_sub!(isize);
