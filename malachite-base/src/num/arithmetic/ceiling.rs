use num::arithmetic::traits::{Ceiling, CeilingAssign};

macro_rules! impl_ceiling {
    ($f:ident) => {
        impl Ceiling for $f {
            type Output = $f;

            /// This is a wrapper over the `ceil` functions in the standard library, for example
            /// [this one](f32::ceil).
            #[inline]
            fn ceiling(self) -> $f {
                $f::ceil(self)
            }
        }

        impl CeilingAssign for $f {
            /// Replaces a number with its ceiling.
            ///
            /// A number's ceiling is the smallest integer greater than or equal to the number.
            ///
            /// $x \gets \lceil x \rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::ceiling#ceiling_assign).
            #[inline]
            fn ceiling_assign(&mut self) {
                *self = self.ceiling();
            }
        }
    };
}
apply_to_primitive_floats!(impl_ceiling);
