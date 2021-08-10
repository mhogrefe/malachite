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
            ///
            /// Wrapping only occurs when `$t` is signed, `self` is `$t::MIN`, and `other` is -1.
            /// The "actual" result, -`$t::MIN`, can't be represented and is wrapped back to
            /// `$t::MIN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_div` module.
            #[inline]
            fn wrapping_div_assign(&mut self, other: $t) {
                *self = self.wrapping_div(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_div);
