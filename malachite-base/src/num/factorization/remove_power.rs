// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Parity, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::factorization::traits::{RemovePower, RemovePowerAssign};

fn remove_power_unsigned<T: PrimitiveUnsigned>(mut x: T, y: T) -> (T, u64) {
    assert!(y > T::ONE, "Cannot remove powers of {y}");
    if x == T::ZERO {
        // every power of `y` divides zero, so, as GMP does, leave it alone
        return (x, 0);
    }
    if y == T::TWO {
        // the exponent is just the number of trailing zeros, which beats dividing repeatedly; GMP
        // special-cases a factor of 2 the same way
        let k = x.trailing_zeros();
        return (x >> k, k);
    }
    let mut k = 0;
    loop {
        let (q, r) = x.div_mod(y);
        if r != T::ZERO {
            return (x, k);
        }
        x = q;
        k += 1;
    }
}

fn remove_power_signed<T: PrimitiveSigned + WrappingFrom<<T as UnsignedAbs>::Output>>(
    x: T,
    y: T,
) -> (T, u64)
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    assert!(
        y > T::ONE || y < T::NEGATIVE_ONE,
        "Cannot remove powers of {y}"
    );
    let (abs, k) = remove_power_unsigned(x.unsigned_abs(), y.unsigned_abs());
    // The quotient is the exact division by the signed power: negative when the value is, and
    // negated again when the factor is negative and the power is odd. Only the negative case can
    // reach the magnitude of `T::MIN`, and there it is representable, so the wrapping conversion is
    // exact.
    let q = T::wrapping_from(abs);
    (
        if (x < T::ZERO) != (y < T::ZERO && k.odd()) {
            q.wrapping_neg()
        } else {
            q
        },
        k,
    )
}

macro_rules! impl_remove_power {
    ($t:ident, $f:ident) => {
        impl RemovePower<$t> for $t {
            type Output = $t;

            /// Removes the largest power of a factor from a number, returning the reduced number
            /// together with the exponent of that power.
            ///
            /// If $f^k$ is the largest power of `other` that divides `self`, this returns
            /// $(\text{self}/f^k, k)$. The factor need not be prime. Zero is left alone, with an
            /// exponent of 0, since every power of the factor divides it.
            ///
            /// For signed types the quotient is the exact division by the signed power, so a
            /// negative factor raised to an odd power flips its sign.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0 or 1, or, for signed types, -1: no largest power exists in
            /// those cases.
            ///
            /// # Examples
            /// See [here](super::remove_power#remove_power).
            #[inline]
            fn remove_power(self, other: $t) -> ($t, u64) {
                $f(self, other)
            }
        }

        impl RemovePowerAssign<$t> for $t {
            /// Divides a number by the largest power of a factor that divides it, in place,
            /// returning the exponent of that power.
            ///
            /// The factor need not be prime. Zero is left alone, with an exponent of 0.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0 or 1, or, for signed types, -1.
            ///
            /// # Examples
            /// See [here](super::remove_power#remove_power_assign).
            #[inline]
            fn remove_power_assign(&mut self, other: $t) -> u64 {
                let (q, k) = $f(*self, other);
                *self = q;
                k
            }
        }
    };
}
macro_rules! impl_remove_power_unsigned {
    ($t:ident) => {
        impl_remove_power!($t, remove_power_unsigned);
    };
}
macro_rules! impl_remove_power_signed {
    ($t:ident) => {
        impl_remove_power!($t, remove_power_signed);
    };
}
apply_to_unsigneds!(impl_remove_power_unsigned);
apply_to_signeds!(impl_remove_power_signed);
