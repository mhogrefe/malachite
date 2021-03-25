use num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};

macro_rules! impl_abs {
    ($u:ident, $s:ident) => {
        impl Abs for $s {
            type Output = $s;

            #[inline]
            fn abs(self) -> $s {
                $s::abs(self)
            }
        }

        impl AbsAssign for $s {
            /// Replaces `self` with its absolute value.
            ///
            /// $x \gets |x|$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::abs` module.
            #[inline]
            fn abs_assign(&mut self) {
                *self = self.abs();
            }
        }

        impl UnsignedAbs for $s {
            type Output = $u;

            #[inline]
            fn unsigned_abs(self) -> $u {
                self.unsigned_abs()
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_abs);
