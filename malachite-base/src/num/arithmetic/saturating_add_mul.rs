use comparison::{Max, Min};
use num::arithmetic::traits::{
    SaturatingAddAssign, SaturatingAddMul, SaturatingAddMulAssign, UnsignedAbs,
};
use num::conversion::traits::WrappingFrom;

macro_rules! impl_saturating_add_mul_unsigned {
    ($t:ident) => {
        impl SaturatingAddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`, saturating at the numeric bounds instead of overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddMul;
            ///
            /// assert_eq!(2u8.saturating_add_mul(3, 7), 23);
            /// assert_eq!(2u8.saturating_add_mul(20, 20), 255);
            /// ```
            #[inline]
            fn saturating_add_mul(self, y: $t, z: $t) -> $t {
                self.saturating_add(y.saturating_mul(z))
            }
        }

        impl SaturatingAddMulAssign for $t {
            /// Replaces `self` with `self + y * z`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddMulAssign;
            ///
            /// let mut x = 2u8;
            /// x.saturating_add_mul_assign(3, 7);
            /// assert_eq!(x, 23);
            ///
            /// let mut x = 2u8;
            /// x.saturating_add_mul_assign(20, 20);
            /// assert_eq!(x, 255);
            /// ```
            #[inline]
            fn saturating_add_mul_assign(&mut self, y: $t, z: $t) {
                self.saturating_add_assign(y.saturating_mul(z));
            }
        }
    };
}

impl_saturating_add_mul_unsigned!(u8);
impl_saturating_add_mul_unsigned!(u16);
impl_saturating_add_mul_unsigned!(u32);
impl_saturating_add_mul_unsigned!(u64);
impl_saturating_add_mul_unsigned!(u128);
impl_saturating_add_mul_unsigned!(usize);

macro_rules! impl_saturating_add_mul_signed {
    ($t:ident) => {
        impl SaturatingAddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`, saturating at the numeric bounds instead of overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddMul;
            ///
            /// assert_eq!(127i8.saturating_add_mul(-2, 100), -73);
            /// assert_eq!((-127i8).saturating_add_mul(-2, 100), -128);
            /// ```
            #[inline]
            fn saturating_add_mul(self, y: $t, z: $t) -> $t {
                if y == 0 || z == 0 {
                    return self;
                }
                let x_sign = self >= 0;
                if x_sign == ((y >= 0) == (z >= 0)) {
                    self.saturating_add(y.saturating_mul(z))
                } else {
                    let x = self.unsigned_abs();
                    let product =
                        if let Some(product) = y.unsigned_abs().checked_mul(z.unsigned_abs()) {
                            product
                        } else {
                            return if x_sign { $t::MIN } else { $t::MAX };
                        };
                    let result = $t::wrapping_from(if x_sign {
                        x.wrapping_sub(product)
                    } else {
                        product.wrapping_sub(x)
                    });
                    if x >= product || (x_sign == (result < 0)) {
                        result
                    } else if x_sign {
                        $t::MIN
                    } else {
                        $t::MAX
                    }
                }
            }
        }

        impl SaturatingAddMulAssign for $t {
            /// Replaces `self` with `self + y * z`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAddMulAssign;
            ///
            /// let mut x = 127i8;
            /// x.saturating_add_mul_assign(-2, 100);
            /// assert_eq!(x, -73);
            ///
            /// let mut x = -127i8;
            /// x.saturating_add_mul_assign(-2, 100);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn saturating_add_mul_assign(&mut self, y: $t, z: $t) {
                *self = self.saturating_add_mul(y, z);
            }
        }
    };
}

impl_saturating_add_mul_signed!(i8);
impl_saturating_add_mul_signed!(i16);
impl_saturating_add_mul_signed!(i32);
impl_saturating_add_mul_signed!(i64);
impl_saturating_add_mul_signed!(i128);
impl_saturating_add_mul_signed!(isize);
