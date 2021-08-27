use num::arithmetic::traits::{AddMul, AddMulAssign, WrappingAddMul, WrappingAddMulAssign};

macro_rules! impl_add_mul_primitive_int {
    ($t:ident) => {
        impl AddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`.
            ///
            /// $f(x, y, z) = x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::add_mul` module.
            #[inline]
            fn add_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_add_mul(y, z)
            }
        }

        impl AddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// $x \gets x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::add_mul` module.
            #[inline]
            fn add_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_add_mul_assign(y, z)
            }
        }
    };
}
apply_to_primitive_ints!(impl_add_mul_primitive_int);

macro_rules! impl_add_mul_primitive_float {
    ($t:ident) => {
        impl AddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`.
            ///
            /// $f(x, y, z) = x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::add_mul` module.
            #[inline]
            fn add_mul(self, y: $t, z: $t) -> $t {
                self + y * z
            }
        }

        impl AddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// $x \gets x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::add_mul` module.
            #[inline]
            fn add_mul_assign(&mut self, y: $t, z: $t) {
                *self += y * z;
            }
        }
    };
}
apply_to_primitive_floats!(impl_add_mul_primitive_float);
