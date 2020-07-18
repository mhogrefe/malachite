use num::arithmetic::traits::{
    OverflowingAdd, OverflowingAddAssign, OverflowingAddMul, OverflowingAddMulAssign,
    OverflowingMul, UnsignedAbs, WrappingAdd, WrappingMul,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;

#[inline]
pub fn _overflowing_add_mul_unsigned<T>(x: T, y: T, z: T) -> (T, bool)
where
    T: OverflowingAdd<T, Output = T> + OverflowingMul<T, Output = T>,
{
    let (product, overflow_1) = y.overflowing_mul(z);
    let (result, overflow_2) = x.overflowing_add(product);
    (result, overflow_1 | overflow_2)
}

macro_rules! impl_overflowing_add_mul_unsigned {
    ($t:ident) => {
        impl OverflowingAddMul<$t> for $t {
            type Output = $t;

            /// Computes `self + y * z`, returning a pair consisting of the wrapped value and a
            /// boolean indicating whether an arithmetic overflow would occur.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingAddMul;
            ///
            /// assert_eq!(2u8.overflowing_add_mul(3, 7), (23, false));
            /// assert_eq!(2u8.overflowing_add_mul(20, 20), (146, true));
            /// ```
            #[inline]
            fn overflowing_add_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_add_mul_unsigned(self, y, z)
            }
        }

        impl OverflowingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingAddMulAssign;
            ///
            /// let mut x = 2u8;
            /// assert_eq!(x.overflowing_add_mul_assign(3, 7), false);
            /// assert_eq!(x, 23);
            ///
            /// let mut x = 2u8;
            /// assert_eq!(x.overflowing_add_mul_assign(20, 20), true);
            /// assert_eq!(x, 146);
            /// ```
            #[inline]
            fn overflowing_add_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (product, overflow) = y.overflowing_mul(z);
                self.overflowing_add_assign(product) | overflow
            }
        }
    };
}
apply_to_unsigneds!(impl_overflowing_add_mul_unsigned);

pub fn _overflowing_add_mul_signed<U: PrimitiveInteger, S: Copy + Eq + Ord + Zero>(
    x: S,
    y: S,
    z: S,
) -> (S, bool)
where
    S: OverflowingAdd<S, Output = S>
        + OverflowingMul<S, Output = S>
        + UnsignedAbs<Output = U>
        + WrappingAdd<S, Output = S>
        + WrappingMul<S, Output = S>,
{
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

            /// Computes `self + y * z`, returning a pair consisting of the wrapped value and a
            /// boolean indicating whether an arithmetic overflow would occur.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingAddMul;
            ///
            /// assert_eq!(127i8.overflowing_add_mul(-2, 100), (-73, false));
            /// assert_eq!((-127i8).overflowing_add_mul(-2, 100), (-71, true));
            /// ```
            #[inline]
            fn overflowing_add_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_add_mul_signed(self, y, z)
            }
        }

        impl OverflowingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingAddMulAssign;
            ///
            /// let mut x = 127i8;
            /// assert_eq!(x.overflowing_add_mul_assign(-2, 100), false);
            /// assert_eq!(x, -73);
            ///
            /// let mut x = -127i8;
            /// assert_eq!(x.overflowing_add_mul_assign(-2, 100), true);
            /// assert_eq!(x, -71);
            /// ```
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
