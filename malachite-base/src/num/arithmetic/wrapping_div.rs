use num::arithmetic::traits::{WrappingDiv, WrappingDivAssign};

macro_rules! impl_wrapping_div {
    ($t:ident) => {
        impl WrappingDiv<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_div` functions in the standard library, for
            /// example [this one](u32::wrapping_div).
            #[inline]
            fn wrapping_div(self, other: $t) -> $t {
                $t::wrapping_div(self, other)
            }
        }

        impl WrappingDivAssign<$t> for $t {
            /// Divides a number by another number in place, wrapping around at the boundary of the
            /// type.
            ///
            /// Wrapping only occurs when `Self` is signed, `self` is `Self::MIN`, and `other` is
            /// -1. The "actual" result, `-Self::MIN`, can't be represented and is wrapped back to
            /// `Self::MIN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_div#wrapping_div_assign).
            #[inline]
            fn wrapping_div_assign(&mut self, other: $t) {
                *self = self.wrapping_div(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_div);
