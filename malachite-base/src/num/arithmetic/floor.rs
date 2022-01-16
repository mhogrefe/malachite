use num::arithmetic::traits::{Floor, FloorAssign};

macro_rules! impl_floor {
    ($f:ident) => {
        impl Floor for $f {
            type Output = $f;

            #[inline]
            fn floor(self) -> $f {
                $f::floor(self)
            }
        }

        impl FloorAssign for $f {
            /// Replaces `self` with its floor.
            ///
            /// $x \gets \lfloor x \rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::floor` module.
            #[inline]
            fn floor_assign(&mut self) {
                *self = self.floor();
            }
        }
    };
}
apply_to_primitive_floats!(impl_floor);
