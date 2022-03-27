use num::arithmetic::traits::{OverflowingPow, OverflowingPowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_overflowing_pow {
    ($t:ident) => {
        impl OverflowingPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_pow(self, exp: u64) -> ($t, bool) {
                $t::overflowing_pow(self, u32::exact_from(exp))
            }
        }

        impl OverflowingPowAssign<u64> for $t {
            /// Replaces `self` with `self ^ exp`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_pow` module.
            #[inline]
            fn overflowing_pow_assign(&mut self, exp: u64) -> bool {
                let overflow;
                (*self, overflow) = OverflowingPow::overflowing_pow(*self, exp);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_pow);
