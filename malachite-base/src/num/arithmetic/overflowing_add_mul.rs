use num::arithmetic::traits::{
    OverflowingAdd, OverflowingAddAssign, OverflowingAddMul, OverflowingAddMulAssign,
    OverflowingMul, UnsignedAbs, WrappingAdd, WrappingMul,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;

fn _overflowing_add_mul_unsigned<
    T: OverflowingAdd<T, Output = T> + OverflowingMul<T, Output = T>,
>(
    x: T,
    y: T,
    z: T,
) -> (T, bool) {
    let (product, overflow_1) = y.overflowing_mul(z);
    let (result, overflow_2) = x.overflowing_add(product);
    (result, overflow_1 | overflow_2)
}

macro_rules! impl_overflowing_add_mul_unsigned {
    ($t:ident) => {
        impl OverflowingAddMul<$t> for $t {
            type Output = $t;

            /// Calculates $x + yz$.
            ///
            /// Returns a tuple of the result along with a boolean indicating whether an arithmetic
            /// overflow would occur. If an overflow would have occurred, then the wrapped value is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_add_mul` module.
            #[inline]
            fn overflowing_add_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_add_mul_unsigned(self, y, z)
            }
        }

        impl OverflowingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_add_mul` module.
            #[inline]
            fn overflowing_add_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (product, overflow) = y.overflowing_mul(z);
                self.overflowing_add_assign(product) | overflow
            }
        }
    };
}
apply_to_unsigneds!(impl_overflowing_add_mul_unsigned);

fn _overflowing_add_mul_signed<
    U: PrimitiveInt,
    S: Copy
        + Eq
        + Ord
        + OverflowingAdd<S, Output = S>
        + OverflowingMul<S, Output = S>
        + UnsignedAbs<Output = U>
        + WrappingAdd<S, Output = S>
        + WrappingMul<S, Output = S>
        + Zero,
>(
    x: S,
    y: S,
    z: S,
) -> (S, bool) {
    if y == S::ZERO || z == S::ZERO {
        return (x, false);
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) == (z >= S::ZERO)) {
        let (product, overflow_1) = y.overflowing_mul(z);
        let (result, overflow_2) = x.overflowing_add(product);
        (result, overflow_1 | overflow_2)
    } else {
        let result = x.wrapping_add(y.wrapping_mul(z));
        let overflow = {
            let x = x.unsigned_abs();
            match y.unsigned_abs().checked_mul(z.unsigned_abs()) {
                Some(product) => {
                    x < product
                        && if x_sign {
                            !x.wrapping_sub(product).get_highest_bit()
                        } else {
                            product.wrapping_sub(x).get_highest_bit()
                        }
                }
                None => true,
            }
        };
        (result, overflow)
    }
}

macro_rules! impl_overflowing_add_mul_signed {
    ($t:ident) => {
        impl OverflowingAddMul<$t> for $t {
            type Output = $t;

            /// Calculates $x + yz$.
            ///
            /// Returns a tuple of the result along with a boolean indicating whether an arithmetic
            /// overflow would occur. If an overflow would have occurred, then the wrapped value is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_add_mul` module.
            #[inline]
            fn overflowing_add_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_add_mul_signed(self, y, z)
            }
        }

        impl OverflowingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::overflowing_add_mul` module.
            #[inline]
            fn overflowing_add_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (result, overflow) = self.overflowing_add_mul(y, z);
                *self = result;
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_add_mul_signed);
