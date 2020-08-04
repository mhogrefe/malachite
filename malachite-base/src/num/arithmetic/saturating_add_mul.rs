use comparison::traits::{Max, Min};
use num::arithmetic::traits::{
    CheckedMul, SaturatingAdd, SaturatingAddAssign, SaturatingAddMul, SaturatingAddMulAssign,
    SaturatingMul, UnsignedAbs, WrappingSub,
};
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

fn _saturating_add_mul_unsigned<T>(x: T, y: T, z: T) -> T
where
    T: SaturatingAdd<T, Output = T> + SaturatingMul<T, Output = T>,
{
    x.saturating_add(y.saturating_mul(z))
}

fn _saturating_add_mul_assign_unsigned<T>(x: &mut T, y: T, z: T)
where
    T: SaturatingAddAssign<T> + SaturatingMul<T, Output = T>,
{
    x.saturating_add_assign(y.saturating_mul(z));
}

macro_rules! impl_saturating_add_mul_unsigned {
    ($t:ident) => {
        impl SaturatingAddMul<$t> for $t {
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
                _saturating_add_mul_unsigned(self, y, z)
            }
        }

        impl SaturatingAddMulAssign<$t> for $t {
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
                _saturating_add_mul_assign_unsigned(self, y, z);
            }
        }
    };
}
apply_to_unsigneds!(impl_saturating_add_mul_unsigned);

fn _saturating_add_mul_signed<U: Copy + Ord, S: Copy + Max + Min + Ord + Zero>(
    x: S,
    y: S,
    z: S,
) -> S
where
    U: CheckedMul<U, Output = U> + WrappingSub<U, Output = U>,
    S: SaturatingAdd<S, Output = S>
        + SaturatingMul<S, Output = S>
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>,
{
    if y == S::ZERO || z == S::ZERO {
        return x;
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) == (z >= S::ZERO)) {
        x.saturating_add(y.saturating_mul(z))
    } else {
        let x = x.unsigned_abs();
        let product = if let Some(product) = y.unsigned_abs().checked_mul(z.unsigned_abs()) {
            product
        } else {
            return if x_sign { S::MIN } else { S::MAX };
        };
        let result = S::wrapping_from(if x_sign {
            x.wrapping_sub(product)
        } else {
            product.wrapping_sub(x)
        });
        if x >= product || (x_sign == (result < S::ZERO)) {
            result
        } else if x_sign {
            S::MIN
        } else {
            S::MAX
        }
    }
}

macro_rules! impl_saturating_add_mul_signed {
    ($t:ident) => {
        impl SaturatingAddMul<$t> for $t {
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
                _saturating_add_mul_signed(self, y, z)
            }
        }

        impl SaturatingAddMulAssign<$t> for $t {
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
apply_to_signeds!(impl_saturating_add_mul_signed);
