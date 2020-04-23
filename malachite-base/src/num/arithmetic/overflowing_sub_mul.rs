use num::arithmetic::traits::{
    OverflowingSubAssign, OverflowingSubMul, OverflowingSubMulAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_overflowing_sub_mul_unsigned {
    ($t:ident) => {
        impl OverflowingSubMul for $t {
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
                let (product, overflow_1) = y.overflowing_mul(z);
                let (result, overflow_2) = self.overflowing_sub(product);
                (result, overflow_1 | overflow_2)
            }
        }

        impl OverflowingSubMulAssign for $t {
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

impl_overflowing_sub_mul_unsigned!(u8);
impl_overflowing_sub_mul_unsigned!(u16);
impl_overflowing_sub_mul_unsigned!(u32);
impl_overflowing_sub_mul_unsigned!(u64);
impl_overflowing_sub_mul_unsigned!(u128);
impl_overflowing_sub_mul_unsigned!(usize);

macro_rules! impl_overflowing_sub_mul_signed {
    ($t:ident) => {
        impl OverflowingSubMul for $t {
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
            #[allow(unstable_name_collisions)]
            fn overflowing_sub_mul(self, y: $t, z: $t) -> ($t, bool) {
                if y == 0 || z == 0 {
                    return (self, false);
                }
                let x_sign = self >= 0;
                if x_sign == ((y >= 0) != (z >= 0)) {
                    let (product, overflow_1) = y.overflowing_mul(z);
                    let (result, overflow_2) = self.overflowing_sub(product);
                    (result, overflow_1 | overflow_2)
                } else {
                    let result = self.wrapping_sub(y.wrapping_mul(z));
                    let overflow = {
                        let x = self.unsigned_abs();
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
        }

        impl OverflowingSubMulAssign for $t {
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

impl_overflowing_sub_mul_signed!(i8);
impl_overflowing_sub_mul_signed!(i16);
impl_overflowing_sub_mul_signed!(i32);
impl_overflowing_sub_mul_signed!(i64);
impl_overflowing_sub_mul_signed!(i128);
impl_overflowing_sub_mul_signed!(isize);
