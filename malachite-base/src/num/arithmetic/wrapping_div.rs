use num::arithmetic::traits::{WrappingDiv, WrappingDivAssign};

macro_rules! impl_wrapping_div {
    ($t:ident) => {
        impl WrappingDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_div(self, other: $t) -> $t {
                $t::wrapping_div(self, other)
            }
        }

        impl WrappingDivAssign<$t> for $t {
            /// Replaces `self` with `self / other`, wrapping around at the boundary of the type.
            /// Wrapping only occurs when `$t` is signed, `self` is `$t::MIN`, and `other` is -1.
            /// The "actual" result, -`$t::MIN`, can't be represented and is wrapped back to
            /// `$t::MIN`.
            ///
            /// Time: worst case O(1)
            ///
            /// Divitional memory: worst case O(1)
            ///
            /// # Examples
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
            fn wrapping_div_assign(&mut self, other: $t) {
                *self = self.wrapping_div(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_div);
