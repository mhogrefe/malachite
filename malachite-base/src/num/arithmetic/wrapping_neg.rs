use num::arithmetic::traits::{WrappingNeg, WrappingNegAssign};

macro_rules! impl_wrapping_neg {
    ($t:ident) => {
        impl WrappingNeg for $t {
            type Output = $t;

            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingNegAssign for $t {
            /// Replaces `self` with its negative, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv -x \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_neg` module.
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_neg);
