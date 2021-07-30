use num::arithmetic::traits::{SubMul, SubMulAssign, WrappingSubMul, WrappingSubMulAssign};

macro_rules! impl_sub_mul {
    ($t:ident) => {
        impl SubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`.
            ///
            /// $f(x, y, z) = x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sub_mul` module.
            #[inline]
            fn sub_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_sub_mul(y, z)
            }
        }

        impl SubMulAssign<$t> for $t {
            /// Replaces `self` with `self - y * z`.
            ///
            /// $x \gets x - yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sub_mul` module.
            #[inline]
            fn sub_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_sub_mul_assign(y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_sub_mul);
