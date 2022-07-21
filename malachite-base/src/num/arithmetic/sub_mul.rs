use crate::num::arithmetic::traits::{SubMul, SubMulAssign, WrappingSubMul, WrappingSubMulAssign};

macro_rules! impl_sub_mul_primitive_int {
    ($t:ident) => {
        impl SubMul<$t> for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers.
            ///
            /// $f(x, y, z) = x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sub_mul#sub_mul).
            #[inline]
            fn sub_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_sub_mul(y, z)
            }
        }

        impl SubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers in place.
            ///
            /// $x \gets x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sub_mul#sub_mul_assign).
            #[inline]
            fn sub_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_sub_mul_assign(y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_sub_mul_primitive_int);

macro_rules! impl_sub_mul_primitive_float {
    ($t:ident) => {
        impl SubMul for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers.
            ///
            /// $f(x, y, z) = x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sub_mul#sub_mul).
            #[inline]
            fn sub_mul(self, y: $t, z: $t) -> $t {
                self - y * z
            }
        }

        impl SubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers in place.
            ///
            /// $x \gets x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sub_mul#sub_mul_assign).
            #[inline]
            fn sub_mul_assign(&mut self, y: $t, z: $t) {
                *self -= y * z;
            }
        }
    };
}
apply_to_primitive_floats!(impl_sub_mul_primitive_float);
