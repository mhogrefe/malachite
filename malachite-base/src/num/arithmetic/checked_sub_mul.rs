use num::arithmetic::traits::{CheckedSubMul, UnsignedAbs};
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

fn checked_sub_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> Option<T> {
    y.checked_mul(z).and_then(|yz| x.checked_sub(yz))
}

macro_rules! impl_checked_sub_mul_unsigned {
    ($t:ident) => {
        impl CheckedSubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`, returning `None` if there is no valid result.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::checked_sub_mul` module.
            #[inline]
            fn checked_sub_mul(self, y: $t, z: $t) -> Option<$t> {
                checked_sub_mul_unsigned(self, y, z)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_sub_mul_unsigned);

fn checked_sub_mul_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    y: T,
    z: T,
) -> Option<T>
where
    U: Ord,
{
    if y == T::ZERO || z == T::ZERO {
        return Some(x);
    }
    let x_sign = x >= T::ZERO;
    if x_sign == ((y >= T::ZERO) != (z >= T::ZERO)) {
        x.checked_sub(y.checked_mul(z)?)
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

macro_rules! impl_checked_sub_mul_signed {
    ($t:ident) => {
        impl CheckedSubMul<$t> for $t {
            type Output = $t;

            /// Computes `self - y * z`, returning `None` if there is no valid result.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::checked_sub_mul` module.
            #[inline]
            fn checked_sub_mul(self, y: $t, z: $t) -> Option<$t> {
                checked_sub_mul_signed(self, y, z)
            }
        }
    };
}
apply_to_signeds!(impl_checked_sub_mul_signed);
