use crate::num::arithmetic::traits::{
    SaturatingMulAssign, SaturatingSquare, SaturatingSquareAssign,
};

macro_rules! impl_saturating_square {
    ($t:ident) => {
        impl SaturatingSquare for $t {
            type Output = $t;

            /// Squares a number, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     x^2 & \text{if} \\quad x^2 \leq M, \\\\
            ///     M & \text{if} \\quad x^2 > M,
            /// \\end{cases}
            /// $$
            /// where $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_square#saturating_square).
            #[inline]
            fn saturating_square(self) -> $t {
                self.saturating_mul(self)
            }
        }

        impl SaturatingSquareAssign for $t {
            /// Squares a number in place, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x^2 & \text{if} \\quad x^2 \leq M, \\\\
            ///     M & \text{if} \\quad x^2 > M,
            /// \\end{cases}
            /// $$
            /// where $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_square#saturating_square_assign).
            #[inline]
            fn saturating_square_assign(&mut self) {
                self.saturating_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_square);
