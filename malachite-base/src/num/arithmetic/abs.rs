use num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};

macro_rules! impl_abs_primitive_int {
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
apply_to_unsigned_signed_pair!(impl_abs_primitive_int);

macro_rules! impl_abs_primitive_float {
    ($f:ident) => {
        impl Abs for $f {
            type Output = $f;

            #[inline]
            fn abs(self) -> $f {
                $f::abs(self)
            }
        }

        impl AbsAssign for $f {
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
    };
}
apply_to_primitive_floats!(impl_abs_primitive_float);
