use num::arithmetic::traits::{SaturatingAbs, SaturatingAbsAssign};

macro_rules! impl_saturating_abs {
    ($t:ident) => {
        impl SaturatingAbs for $t {
            type Output = $t;

            #[inline]
            fn saturating_abs(self) -> $t {
                $t::saturating_abs(self)
            }
        }

        impl SaturatingAbsAssign for $t {
            /// Replace `self` with its absolute value, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     |x| & x > -2^{W-1} \\\\
            ///     2^{W-1} - 1 & x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_abs` module.
            #[inline]
            fn saturating_abs_assign(&mut self) {
                *self = self.saturating_abs();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_abs);
