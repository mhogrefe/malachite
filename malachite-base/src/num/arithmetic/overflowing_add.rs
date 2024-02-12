use crate::num::arithmetic::traits::{OverflowingAdd, OverflowingAddAssign};

macro_rules! impl_overflowing_add {
    ($t:ident) => {
        impl OverflowingAdd<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_add` functions in the standard library, for
            /// example [this one](u32::overflowing_add).
            #[inline]
            fn overflowing_add(self, other: $t) -> ($t, bool) {
                $t::overflowing_add(self, other)
            }
        }

        impl OverflowingAddAssign<$t> for $t {
            /// Adds a number to another number, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_add#overflowing_add_assign).
            #[inline]
            fn overflowing_add_assign(&mut self, other: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_add(other);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_add);
