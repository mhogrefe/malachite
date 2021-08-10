use num::arithmetic::traits::{WrappingMulAssign, WrappingSquare, WrappingSquareAssign};

macro_rules! impl_wrapping_square {
    ($t:ident) => {
        impl WrappingSquare for $t {
            type Output = $t;

            /// Squares `self`, wrapping around at the boundary of the type.
            ///
            /// $f(x) = y$, where $y \equiv x^2 \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_square` module.
            #[inline]
            fn wrapping_square(self) -> $t {
                self.wrapping_mul(self)
            }
        }

        impl WrappingSquareAssign for $t {
            /// Replaces `self` with `self` squared, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv x^2 \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_square` module.
            #[inline]
            fn wrapping_square_assign(&mut self) {
                self.wrapping_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_square);
