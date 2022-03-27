use num::arithmetic::traits::{OverflowingSub, OverflowingSubAssign};

macro_rules! impl_overflowing_sub {
    ($t:ident) => {
        impl OverflowingSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_sub(self, other: $t) -> ($t, bool) {
                $t::overflowing_sub(self, other)
            }
        }

        impl OverflowingSubAssign<$t> for $t {
            /// Replaces `self` with `self - other`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_sub` module.
            #[inline]
            fn overflowing_sub_assign(&mut self, other: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_sub(other);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_sub);
