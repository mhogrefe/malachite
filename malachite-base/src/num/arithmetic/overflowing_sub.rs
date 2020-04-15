use num::arithmetic::traits::{OverflowingSub, OverflowingSubAssign};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl OverflowingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_sub(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_sub(self, rhs)
            }
        }

        impl OverflowingSubAssign for $t {
            /// Replaces `self` with `self - rhs`.
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
            fn overflowing_sub_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_sub(rhs);
                *self = result;
                overflow
            }
        }
    };
}

impl_arithmetic_traits!(u8);
impl_arithmetic_traits!(u16);
impl_arithmetic_traits!(u32);
impl_arithmetic_traits!(u64);
impl_arithmetic_traits!(u128);
impl_arithmetic_traits!(usize);
impl_arithmetic_traits!(i8);
impl_arithmetic_traits!(i16);
impl_arithmetic_traits!(i32);
impl_arithmetic_traits!(i64);
impl_arithmetic_traits!(i128);
impl_arithmetic_traits!(isize);
