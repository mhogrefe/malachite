use num::arithmetic::traits::{SaturatingNeg, SaturatingNegAssign};

macro_rules! impl_saturating_neg {
    ($t:ident) => {
        impl SaturatingNeg for $t {
            type Output = $t;

            #[inline]
            fn saturating_neg(self) -> $t {
                $t::saturating_neg(self)
            }
        }

        impl SaturatingNegAssign for $t {
            /// Replaces `self` with its negative, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     -x & x^2 > -2^{W-1} \\\\
            ///     2^{W-1} - 1 & x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::saturating_neg` module.
            #[inline]
            fn saturating_neg_assign(&mut self) {
                *self = self.saturating_neg();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_neg);
