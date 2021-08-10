use num::arithmetic::traits::{
    WrappingAdd, WrappingAddAssign, WrappingAddMul, WrappingAddMulAssign, WrappingMul,
};

fn _wrapping_add_mul<T: WrappingAdd<T, Output = T> + WrappingMul<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.wrapping_add(y.wrapping_mul(z))
}

fn _wrapping_add_mul_assign<T: WrappingAddAssign<T> + WrappingMul<T, Output = T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.wrapping_add_assign(y.wrapping_mul(z));
}

macro_rules! impl_wrapping_add_mul {
    ($t:ident) => {
        impl WrappingAddMul<$t> for $t {
            type Output = $t;

            /// Computes $x + yz$, wrapping around at the boundary of the type.
            ///
            /// $f(x, y, z) = w$, where $w \equiv x + yz \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_add_mul` module.
            #[inline]
            fn wrapping_add_mul(self, y: $t, z: $t) -> $t {
                _wrapping_add_mul(self, y, z)
            }
        }

        impl WrappingAddMulAssign<$t> for $t {
            /// Replaces $x$ with $x + yz$, wrapping around at the boundary of the type.
            ///
            /// $x \gets w$, where $w \equiv x + yz \mod 2^W$ and $W$ is `$t::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::wrapping_add_mul` module.
            #[inline]
            fn wrapping_add_mul_assign(&mut self, y: $t, z: $t) {
                _wrapping_add_mul_assign(self, y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add_mul);
