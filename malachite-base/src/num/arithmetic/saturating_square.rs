use num::arithmetic::traits::{SaturatingMulAssign, SaturatingSquare, SaturatingSquareAssign};

macro_rules! impl_saturating_square {
    ($t:ident) => {
        impl SaturatingSquare for $t {
            type Output = $t;

            /// Squares `self`, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     x^2 & x^2 \leq M \\\\
            ///     M & x^2 > M,
            /// \\end{cases}
            /// $$
            /// where $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_square` module.
            #[inline]
            fn saturating_square(self) -> $t {
                self.saturating_mul(self)
            }
        }

        impl SaturatingSquareAssign for $t {
            /// Replaces `self` with `self ^ 2`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x^2 & x^2 \leq M \\\\
            ///     M & x^2 > M,
            /// \\end{cases}
            /// $$
            /// where $M$ is `$t::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_square` module.
            #[inline]
            fn saturating_square_assign(&mut self) {
                self.saturating_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_square);
