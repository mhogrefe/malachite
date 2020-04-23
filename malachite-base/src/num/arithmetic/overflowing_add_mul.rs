use num::arithmetic::traits::{
    OverflowingAddAssign, OverflowingAddMul, OverflowingAddMulAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_overflowing_add_mul_unsigned {
    ($t:ident) => {
        impl OverflowingAddMul for $t {
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
                let (product, overflow_1) = y.overflowing_mul(z);
                let (result, overflow_2) = self.overflowing_add(product);
                (result, overflow_1 | overflow_2)
            }
        }

        impl OverflowingAddMulAssign for $t {
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

impl_overflowing_add_mul_unsigned!(u8);
impl_overflowing_add_mul_unsigned!(u16);
impl_overflowing_add_mul_unsigned!(u32);
impl_overflowing_add_mul_unsigned!(u64);
impl_overflowing_add_mul_unsigned!(u128);
impl_overflowing_add_mul_unsigned!(usize);

macro_rules! impl_overflowing_add_mul_signed {
    ($t:ident) => {
        impl OverflowingAddMul for $t {
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
            #[allow(unstable_name_collisions)]
            fn overflowing_add_mul(self, y: $t, z: $t) -> ($t, bool) {
                if y == 0 || z == 0 {
                    return (self, false);
                }
                let x_sign = self >= 0;
                if x_sign == ((y >= 0) == (z >= 0)) {
                    let (product, overflow_1) = y.overflowing_mul(z);
                    let (result, overflow_2) = self.overflowing_add(product);
                    (result, overflow_1 | overflow_2)
                } else {
                    let result = self.wrapping_add(y.wrapping_mul(z));
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

        impl OverflowingAddMulAssign for $t {
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

impl_overflowing_add_mul_signed!(i8);
impl_overflowing_add_mul_signed!(i16);
impl_overflowing_add_mul_signed!(i32);
impl_overflowing_add_mul_signed!(i64);
impl_overflowing_add_mul_signed!(i128);
impl_overflowing_add_mul_signed!(isize);
