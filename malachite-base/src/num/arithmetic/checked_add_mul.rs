use num::arithmetic::traits::{CheckedAdd, CheckedAddMul, CheckedMul, UnsignedAbs, WrappingSub};
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

fn _checked_add_mul_unsigned<T: CheckedAdd<T, Output = T> + CheckedMul<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> Option<T> {
    y.checked_mul(z).and_then(|yz| x.checked_add(yz))
}

macro_rules! impl_checked_add_mul_unsigned {
    ($t:ident) => {
        impl CheckedAddMul<$t> for $t {
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
                _checked_add_mul_unsigned(self, y, z)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_add_mul_unsigned);

fn _checked_add_mul_signed<
    U: CheckedMul<U, Output = U> + Copy + Ord + WrappingSub<U, Output = U>,
    T: CheckedAdd<T, Output = T>
        + CheckedMul<T, Output = T>
        + Copy
        + Ord
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>
        + Zero,
>(
    x: T,
    y: T,
    z: T,
) -> Option<T> {
    if y == T::ZERO || z == T::ZERO {
        return Some(x);
    }
    let x_sign = x >= T::ZERO;
    if x_sign == ((y >= T::ZERO) == (z >= T::ZERO)) {
        x.checked_add(y.checked_mul(z)?)
    } else {
        let x = x.unsigned_abs();
        let product = y.unsigned_abs().checked_mul(z.unsigned_abs())?;
        let result = T::wrapping_from(if x_sign {
            x.wrapping_sub(product)
        } else {
            product.wrapping_sub(x)
        });
        if x >= product || (x_sign == (result < T::ZERO)) {
            Some(result)
        } else {
            None
        }
    }
}

macro_rules! impl_checked_add_mul_signed {
    ($t:ident) => {
        impl CheckedAddMul<$t> for $t {
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
                _checked_add_mul_signed(self, y, z)
            }
        }
    };
}
apply_to_signeds!(impl_checked_add_mul_signed);
