use num::arithmetic::traits::{OverflowingMulAssign, OverflowingSquare, OverflowingSquareAssign};

macro_rules! impl_overflowing_square {
    ($t:ident) => {
        impl OverflowingSquare for $t {
            type Output = $t;

            /// Calculates `self ^ 2`.
            ///
            /// Returns a tuple of the result along with a boolean indicating whether an arithmetic
            /// overflow would occur. If an overflow would have occurred, then the wrapped value is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_square` module.
            #[inline]
            fn overflowing_square(self) -> ($t, bool) {
                self.overflowing_mul(self)
            }
        }

        impl OverflowingSquareAssign for $t {
            /// Replaces `self` with `self ^ 2`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_square` module.
            #[inline]
            fn overflowing_square_assign(&mut self) -> bool {
                self.overflowing_mul_assign(*self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_square);
