use num::arithmetic::traits::{Ceiling, CeilingAssign};

macro_rules! impl_ceiling {
    ($f:ident) => {
        impl Ceiling for $f {
            type Output = $f;

            #[inline]
            fn ceiling(self) -> $f {
                $f::ceil(self)
            }
        }

        impl CeilingAssign for $f {
            /// Replaces `self` with its ceiling.
            ///
            /// $x \gets \lceil x \rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::ceiling` module.
            #[inline]
            fn ceiling_assign(&mut self) {
                *self = self.ceiling();
            }
        }
    };
}
apply_to_primitive_floats!(impl_ceiling);
