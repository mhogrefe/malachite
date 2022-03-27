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
            /// overflow would have occurred, then the wrapped value is assigned. Overflow only
            /// occurs when `$t` is signed, `self` is `$t::MIN`, and `other` is -1. The "actual"
            /// result, -`$t::MIN`, can't be represented and is wrapped back to `$t::MIN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_div` module.
            #[inline]
            fn overflowing_div_assign(&mut self, other: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_div(other);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_div);
