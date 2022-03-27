use num::arithmetic::traits::{OverflowingAbs, OverflowingAbsAssign};

macro_rules! impl_overflowing_abs {
    ($t:ident) => {
        impl OverflowingAbs for $t {
            type Output = $t;

            #[inline]
            fn overflowing_abs(self) -> ($t, bool) {
                $t::overflowing_abs(self)
            }
        }

        impl OverflowingAbsAssign for $t {
            /// Replaces `self` with its absolute value.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_abs` module.
            #[inline]
            fn overflowing_abs_assign(&mut self) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_abs();
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_abs);
