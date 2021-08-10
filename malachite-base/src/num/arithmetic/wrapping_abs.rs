use num::arithmetic::traits::{WrappingAbs, WrappingAbsAssign};

macro_rules! impl_wrapping_abs {
    ($t:ident) => {
        impl WrappingAbs for $t {
            type Output = $t;

            #[inline]
            fn wrapping_abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl WrappingAbsAssign for $t {
            /// Replaces `self` with its absolute value, wrapping around at the boundary of the
            /// type.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     |x| & x > -2^{W-1} \\\\
            ///     -2^{W-1} & x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_abs` module.
            #[inline]
            fn wrapping_abs_assign(&mut self) {
                *self = self.wrapping_abs();
            }
        }
    };
}
apply_to_signeds!(impl_wrapping_abs);
