use num::arithmetic::traits::{SaturatingAdd, SaturatingAddAssign};

macro_rules! impl_saturating_add {
    ($t:ident) => {
        impl SaturatingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn saturating_add(self, other: $t) -> $t {
                $t::saturating_add(self, other)
            }
        }

        impl SaturatingAddAssign<$t> for $t {
            /// Replaces `self` with `self + other`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddAssign;
            ///
            /// let mut x = 123u16;
            /// x.saturating_add_assign(456);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// x.saturating_add_assign(200);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_add_assign(&mut self, other: $t) {
                *self = self.saturating_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_add);
