use crate::num::arithmetic::traits::{
    RotateLeft, RotateLeftAssign, RotateRight, RotateRightAssign,
};
use crate::num::conversion::traits::WrappingFrom;

macro_rules! impl_rotate {
    ($t:ident) => {
        impl RotateLeft for $t {
            type Output = $t;

            /// This is a wrapper over the `rotate_left` functions in the standard library, for
            /// example [this one](u32::rotate_left).
            #[inline]
            fn rotate_left(self, n: u64) -> $t {
                $t::rotate_left(self, u32::wrapping_from(n))
            }
        }

        impl RotateLeftAssign for $t {
            /// Rotates a number left, in place.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::rotate#rotate_left_assign).
            #[inline]
            fn rotate_left_assign(&mut self, n: u64) {
                *self = self.rotate_left(u32::wrapping_from(n));
            }
        }

        impl RotateRight for $t {
            type Output = $t;

            /// This is a wrapper over the `rotate_right` functions in the standard library, for
            /// example [this one](u32::rotate_right).
            #[inline]
            fn rotate_right(self, n: u64) -> $t {
                $t::rotate_right(self, u32::wrapping_from(n))
            }
        }

        impl RotateRightAssign for $t {
            /// Rotates a number right, in place.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::rotate#rotate_right_assign).
            #[inline]
            fn rotate_right_assign(&mut self, n: u64) {
                *self = self.rotate_right(u32::wrapping_from(n));
            }
        }
    };
}
apply_to_primitive_ints!(impl_rotate);
