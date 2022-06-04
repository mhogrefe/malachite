use num::arithmetic::traits::{Floor, FloorAssign};

macro_rules! impl_floor {
    ($f:ident) => {
        impl Floor for $f {
            type Output = $f;

            /// This is a wrapper over the `floor` functions in the standard library, for example
            /// [this one](f32::floor).
            #[inline]
            fn floor(self) -> $f {
                $f::floor(self)
            }
        }

        impl FloorAssign for $f {
            /// Replaces a number with its floor.
            ///
            /// A number's floor is the largest integer less than or equal to the number.
            ///
            /// $x \gets \lfloor x \rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::floor#floor_assign).
            #[inline]
            fn floor_assign(&mut self) {
                *self = self.floor();
            }
        }
    };
}
apply_to_primitive_floats!(impl_floor);
