use num::arithmetic::traits::{
    OverflowingMul, OverflowingSub, OverflowingSubAssign, OverflowingSubMul,
    OverflowingSubMulAssign, UnsignedAbs, WrappingMul, WrappingSub,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;

fn _overflowing_sub_mul_unsigned<T>(x: T, y: T, z: T) -> (T, bool)
where
    T: OverflowingMul<T, Output = T> + OverflowingSub<T, Output = T>,
{
    let (product, overflow_1) = y.overflowing_mul(z);
    let (result, overflow_2) = x.overflowing_sub(product);
    (result, overflow_1 | overflow_2)
}

macro_rules! impl_overflowing_sub_mul_unsigned {
    ($t:ident) => {
        impl OverflowingSubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`, returning a pair consisting of the wrapped value and a
            /// boolean indicating whether an arithmetic overflow would occur.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSubMul;
            ///
            /// assert_eq!(60u8.overflowing_sub_mul(5, 10), (10, false));
            /// assert_eq!(2u8.overflowing_sub_mul(10, 5), (208, true));
            /// ```
            #[inline]
            fn overflowing_sub_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_sub_mul_unsigned(self, y, z)
            }
        }

        impl OverflowingSubMulAssign<$t> for $t {
            /// Replaces `self` with `self - y * z`.
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
            /// use malachite_base::num::arithmetic::traits::OverflowingSubMulAssign;
            ///
            /// let mut x = 60u8;
            /// assert_eq!(x.overflowing_sub_mul_assign(5, 10), false);
            /// assert_eq!(x, 10);
            ///
            /// let mut x = 2u8;
            /// assert_eq!(x.overflowing_sub_mul_assign(10, 5), true);
            /// assert_eq!(x, 208);
            /// ```
            #[inline]
            fn overflowing_sub_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (product, overflow) = y.overflowing_mul(z);
                self.overflowing_sub_assign(product) | overflow
            }
        }
    };
}
apply_to_unsigneds!(impl_overflowing_sub_mul_unsigned);

fn _overflowing_sub_mul<U: PrimitiveInteger, S: Copy + Ord + Zero>(x: S, y: S, z: S) -> (S, bool)
where
    S: OverflowingMul<S, Output = S>
        + OverflowingSub<S, Output = S>
        + UnsignedAbs<Output = U>
        + WrappingMul<S, Output = S>
        + WrappingSub<S, Output = S>,
{
    if y == S::ZERO || z == S::ZERO {
        return (x, false);
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) != (z >= S::ZERO)) {
        let (product, overflow_1) = y.overflowing_mul(z);
        let (result, overflow_2) = x.overflowing_sub(product);
        (result, overflow_1 | overflow_2)
    } else {
        let result = x.wrapping_sub(y.wrapping_mul(z));
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

macro_rules! impl_overflowing_sub_mul_signed {
    ($t:ident) => {
        impl OverflowingSubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`, returning a pair consisting of the wrapped value and a
            /// boolean indicating whether an arithmetic overflow would occur.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSubMul;
            ///
            /// assert_eq!(127i8.overflowing_sub_mul(2, 100), (-73, false));
            /// assert_eq!((-127i8).overflowing_sub_mul(2, 100), (-71, true));
            /// ```
            #[inline]
            fn overflowing_sub_mul(self, y: $t, z: $t) -> ($t, bool) {
                _overflowing_sub_mul(self, y, z)
            }
        }

        impl OverflowingSubMulAssign<$t> for $t {
            /// Replaces `self` with `self - y * z`.
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
            /// use malachite_base::num::arithmetic::traits::OverflowingSubMulAssign;
            ///
            /// let mut x = 127i8;
            /// assert_eq!(x.overflowing_sub_mul_assign(2, 100), false);
            /// assert_eq!(x, -73);
            ///
            /// let mut x = -127i8;
            /// assert_eq!(x.overflowing_sub_mul_assign(2, 100), true);
            /// assert_eq!(x, -71);
            /// ```
            #[inline]
            fn overflowing_sub_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (result, overflow) = self.overflowing_sub_mul(y, z);
                *self = result;
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_sub_mul_signed);
