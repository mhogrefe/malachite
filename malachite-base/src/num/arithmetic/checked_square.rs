use num::arithmetic::traits::CheckedSquare;

macro_rules! impl_checked_square {
    ($t:ident) => {
        impl CheckedSquare for $t {
            type Output = $t;

            /// Squares `self`, returning `None` if there is no valid result.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(x^2) & x^2 < 2^W \\\\
            ///     \operatorname{None} & x^2 \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::checked_square` module.
            #[inline]
            fn checked_square(self) -> Option<$t> {
                self.checked_mul(self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_square);
