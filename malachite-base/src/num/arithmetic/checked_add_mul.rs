use num::arithmetic::traits::{CheckedAddMul, UnsignedAbs};
use num::conversion::traits::WrappingFrom;

macro_rules! impl_checked_add_mul_unsigned {
    ($t:ident) => {
        impl CheckedAddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`, returning `None` if there is no valid result.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::CheckedAddMul;
            ///
            /// assert_eq!(2u8.checked_add_mul(3, 7), Some(23));
            /// assert_eq!(2u8.checked_add_mul(20, 20), None);
            /// ```
            #[inline]
            fn checked_add_mul(self, y: $t, z: $t) -> Option<$t> {
                y.checked_mul(z).and_then(|yz| self.checked_add(yz))
            }
        }
    };
}

impl_checked_add_mul_unsigned!(u8);
impl_checked_add_mul_unsigned!(u16);
impl_checked_add_mul_unsigned!(u32);
impl_checked_add_mul_unsigned!(u64);
impl_checked_add_mul_unsigned!(u128);
impl_checked_add_mul_unsigned!(usize);

macro_rules! impl_checked_add_mul_signed {
    ($t:ident) => {
        impl CheckedAddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`, returning `None` if there is no valid result.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::CheckedAddMul;
            ///
            /// assert_eq!(127i8.checked_add_mul(-2, 100), Some(-73));
            /// assert_eq!((-127i8).checked_add_mul(-2, 100), None);
            /// ```
            #[inline]
            fn checked_add_mul(self, y: $t, z: $t) -> Option<$t> {
                if y == 0 || z == 0 {
                    return Some(self);
                }
                let x_sign = self >= 0;
                if x_sign == ((y >= 0) == (z >= 0)) {
                    self.checked_add(y.checked_mul(z)?)
                } else {
                    let x = self.unsigned_abs();
                    let product = y.unsigned_abs().checked_mul(z.unsigned_abs())?;
                    let result = $t::wrapping_from(if x_sign {
                        x.wrapping_sub(product)
                    } else {
                        product.wrapping_sub(x)
                    });
                    if x >= product || (x_sign == (result < 0)) {
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_checked_add_mul_signed!(i8);
impl_checked_add_mul_signed!(i16);
impl_checked_add_mul_signed!(i32);
impl_checked_add_mul_signed!(i64);
impl_checked_add_mul_signed!(i128);
impl_checked_add_mul_signed!(isize);
