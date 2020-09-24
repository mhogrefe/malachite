use num::arithmetic::traits::{WrappingPow, WrappingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_wrapping_pow {
    ($t:ident) => {
        impl WrappingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_pow(self, exp: u64) -> $t {
                $t::wrapping_pow(self, u32::exact_from(exp))
            }
        }

        impl WrappingPowAssign<u64> for $t {
            /// Replaces `self` with `self ^ exp`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingPowAssign;
            ///
            /// let mut x = 3u8;
            /// x.wrapping_pow_assign(3);
            /// assert_eq!(x, 27);
            ///
            /// let mut x = -10i32;
            /// x.wrapping_pow_assign(9);
            /// assert_eq!(x, -1000000000);
            ///
            /// let mut x = -10i16;
            /// x.wrapping_pow_assign(9);
            /// assert_eq!(x, 13824);
            /// ```
            #[inline]
            fn wrapping_pow_assign(&mut self, exp: u64) {
                *self = WrappingPow::wrapping_pow(*self, exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_pow);
