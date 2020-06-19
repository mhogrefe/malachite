use num::arithmetic::traits::{OverflowingDiv, OverflowingDivAssign};

macro_rules! impl_overflowing_div {
    ($t:ident) => {
        impl OverflowingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_div(self, other: $t) -> ($t, bool) {
                $t::overflowing_div(self, other)
            }
        }

        impl OverflowingDivAssign<$t> for $t {
            /// Replaces `self` with `self / other`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned. Overflow only
            /// occurs when `$t` is signed, `self` is `$t::MIN`, and `other` is -1. The "actual"
            /// result, -`$t::MIN`, can't be represented and is wrapped back to `$t::MIN`.
            ///
            /// Time: worst case O(1)
            ///
            /// Divitional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingDivAssign;
            ///
            /// let mut x = 100u16;
            /// assert_eq!(x.overflowing_div_assign(3), false);
            /// assert_eq!(x, 33);
            ///
            /// let mut x = -128i8;
            /// assert_eq!(x.overflowing_div_assign(-1), true);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn overflowing_div_assign(&mut self, other: $t) -> bool {
                let (result, overflow) = self.overflowing_div(other);
                *self = result;
                overflow
            }
        }
    };
}

impl_overflowing_div!(u8);
impl_overflowing_div!(u16);
impl_overflowing_div!(u32);
impl_overflowing_div!(u64);
impl_overflowing_div!(u128);
impl_overflowing_div!(usize);
impl_overflowing_div!(i8);
impl_overflowing_div!(i16);
impl_overflowing_div!(i32);
impl_overflowing_div!(i64);
impl_overflowing_div!(i128);
impl_overflowing_div!(isize);
