use num::arithmetic::traits::{OverflowingPow, OverflowingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_overflowing_pow {
    ($t:ident) => {
        impl OverflowingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_pow(self, exp: u64) -> ($t, bool) {
                $t::overflowing_pow(self, u32::exact_from(exp))
            }
        }

        impl OverflowingPowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)`.
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
            /// use malachite_base::num::arithmetic::traits::OverflowingPowAssign;
            ///
            /// let mut x = 3u8;
            /// assert_eq!(x.overflowing_pow_assign(3), false);
            /// assert_eq!(x, 27);
            ///
            /// let mut x = -10i32;
            /// assert_eq!(x.overflowing_pow_assign(9), false);
            /// assert_eq!(x, -1_000_000_000);
            ///
            /// let mut x = -10i16;
            /// assert_eq!(x.overflowing_pow_assign(9), true);
            /// assert_eq!(x, 13_824);
            /// ```
            #[inline]
            fn overflowing_pow_assign(&mut self, exp: u64) -> bool {
                let (pow, overflow) = OverflowingPow::overflowing_pow(*self, exp);
                *self = pow;
                overflow
            }
        }
    };
}
impl_overflowing_pow!(u8);
impl_overflowing_pow!(u16);
impl_overflowing_pow!(u32);
impl_overflowing_pow!(u64);
impl_overflowing_pow!(u128);
impl_overflowing_pow!(usize);
impl_overflowing_pow!(i8);
impl_overflowing_pow!(i16);
impl_overflowing_pow!(i32);
impl_overflowing_pow!(i64);
impl_overflowing_pow!(i128);
impl_overflowing_pow!(isize);
