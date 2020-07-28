use num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};

macro_rules! impl_wrapping_add {
    ($t:ident) => {
        impl WrappingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_add(self, other: $t) -> $t {
                $t::wrapping_add(self, other)
            }
        }

        impl WrappingAddAssign<$t> for $t {
            /// Replaces `self` with `self + other`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddAssign;
            ///
            /// let mut x = 123u16;
            /// x.wrapping_add_assign(456);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// x.wrapping_add_assign(200);
            /// assert_eq!(x, 67);
            /// ```
            #[inline]
            fn wrapping_add_assign(&mut self, other: $t) {
                *self = self.wrapping_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add);
