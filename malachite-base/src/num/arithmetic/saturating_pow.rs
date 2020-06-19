use num::arithmetic::traits::{SaturatingPow, SaturatingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_saturating_pow {
    ($t:ident) => {
        impl SaturatingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn saturating_pow(self, exp: u64) -> $t {
                $t::saturating_pow(self, u32::exact_from(exp))
            }
        }

        impl SaturatingPowAssign<u64> for $t {
            /// Replaces `self` with `self ^ exp`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingPowAssign;
            ///
            /// let mut x = 3u8;
            /// x.saturating_pow_assign(3);
            /// assert_eq!(x, 27);
            ///
            /// let mut x = -10i32;
            /// x.saturating_pow_assign(9);
            /// assert_eq!(x, -1_000_000_000);
            ///
            /// let mut x = -10i16;
            /// x.saturating_pow_assign(9);
            /// assert_eq!(x, -32_768);
            /// ```
            #[inline]
            fn saturating_pow_assign(&mut self, exp: u64) {
                *self = SaturatingPow::saturating_pow(*self, exp);
            }
        }
    };
}
impl_saturating_pow!(u8);
impl_saturating_pow!(u16);
impl_saturating_pow!(u32);
impl_saturating_pow!(u64);
impl_saturating_pow!(u128);
impl_saturating_pow!(usize);
impl_saturating_pow!(i8);
impl_saturating_pow!(i16);
impl_saturating_pow!(i32);
impl_saturating_pow!(i64);
impl_saturating_pow!(i128);
impl_saturating_pow!(isize);
