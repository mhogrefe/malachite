use crate::num::arithmetic::traits::{WrappingNeg, WrappingNegAssign};

macro_rules! impl_wrapping_neg {
    ($t:ident) => {
        impl WrappingNeg for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_neg` functions in the standard library, for
            /// example [this one](u32::wrapping_neg).
            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingNegAssign for $t {
            /// Negates a number in place, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv -x \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_neg#wrapping_neg_assign).
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_neg);
