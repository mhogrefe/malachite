use num::arithmetic::traits::{WrappingDiv, WrappingDivAssign};

macro_rules! impl_wrapping_div {
    ($t:ident) => {
        impl WrappingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_div(self, rhs: $t) -> $t {
                $t::wrapping_div(self, rhs)
            }
        }

        impl WrappingDivAssign for $t {
            /// Replaces `self` with `self / rhs`, wrapping around at the boundary of the type.
            /// Wrapping only occurs when `$t` is signed, `self` is `$t::MIN`, and `rhs` is -1. The
            /// "actual" result, -`$t::MIN`, can't be represented and is wrapped back to `$t::MIN`.
            ///
            /// Time: worst case O(1)
            ///
            /// Divitional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingDivAssign;
            ///
            /// let mut x = 100u16;
            /// x.wrapping_div_assign(3);
            /// assert_eq!(x, 33);
            ///
            /// let mut x = -128i8;
            /// x.wrapping_div_assign(-1);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn wrapping_div_assign(&mut self, rhs: $t) {
                *self = self.wrapping_div(rhs);
            }
        }
    };
}

impl_wrapping_div!(u8);
impl_wrapping_div!(u16);
impl_wrapping_div!(u32);
impl_wrapping_div!(u64);
impl_wrapping_div!(u128);
impl_wrapping_div!(usize);
impl_wrapping_div!(i8);
impl_wrapping_div!(i16);
impl_wrapping_div!(i32);
impl_wrapping_div!(i64);
impl_wrapping_div!(i128);
impl_wrapping_div!(isize);
