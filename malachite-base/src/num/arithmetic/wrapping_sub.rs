use num::arithmetic::traits::{WrappingSub, WrappingSubAssign};

macro_rules! impl_wrapping_sub {
    ($t:ident) => {
        impl WrappingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, other: $t) -> $t {
                $t::wrapping_sub(self, other)
            }
        }

        impl WrappingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`, wrapping around at the boundary of the type.
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
            fn wrapping_sub_assign(&mut self, other: $t) {
                *self = self.wrapping_sub(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_sub);
