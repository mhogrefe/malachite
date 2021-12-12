use num::arithmetic::traits::{
    WrappingMul, WrappingSub, WrappingSubAssign, WrappingSubMul, WrappingSubMulAssign,
};

fn wrapping_sub_mul<T: WrappingMul<T, Output = T> + WrappingSub<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.wrapping_sub(y.wrapping_mul(z))
}

fn wrapping_sub_mul_assign<T: WrappingMul<T, Output = T> + WrappingSubAssign<T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.wrapping_sub_assign(y.wrapping_mul(z));
}

macro_rules! impl_wrapping_sub_mul {
    ($t:ident) => {
        impl WrappingSubMul<$t> for $t {
            type Output = $t;

            /// Computes $x - yz$, wrapping around at the boundary of the type.
            ///
            /// $f(x, y, z) = w$, where $w \equiv x - yz \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_sub_mul` module.
            #[inline]
            fn wrapping_sub_mul(self, y: $t, z: $t) -> $t {
                wrapping_sub_mul(self, y, z)
            }
        }

        impl WrappingSubMulAssign<$t> for $t {
            /// Replaces $x$ with $x - yz$, wrapping around at the boundary of the type.
            ///
            /// $x \gets w$, where $w \equiv x - yz \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_sub_mul` module.
            #[inline]
            fn wrapping_sub_mul_assign(&mut self, y: $t, z: $t) {
                wrapping_sub_mul_assign(self, y, z)
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_sub_mul);
