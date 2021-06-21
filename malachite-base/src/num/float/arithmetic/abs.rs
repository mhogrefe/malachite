use num::arithmetic::traits::{Abs, AbsAssign};

//TODO
macro_rules! impl_abs {
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
apply_to_primitive_floats!(impl_abs);
