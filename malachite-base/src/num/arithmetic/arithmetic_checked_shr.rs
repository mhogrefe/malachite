// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use core::ops::Shr;

fn arithmetic_checked_shr_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveUnsigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits < S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let abs_bits = bits.unsigned_abs();
        Some(if abs_bits >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shr_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// non-negative amount; shifting by a large amount returns `Some(0)`.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(\lfloor x/2^b \rfloor) &
                    ///         \text{if} \\quad b \geq 0, \\\\
                    ///     \operatorname{Some}(2^{-b} x) &
                    ///         \text{if} \\quad b < 0 \\ \mathrm{and} \\ 2^{-b} x < 2^W, \\\\
                    ///     \operatorname{None} &
                    ///         \text{if} \\quad b < 0 \\ \mathrm{and} \\ 2^{-b} x \geq 2^W, \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shr#arithmetic_checked_shr).
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shr_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shr_unsigned_signed);

fn arithmetic_checked_shr_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveSigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits < S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let width = U::wrapping_from(T::WIDTH);
        let abs_bits = bits.unsigned_abs();
        Some(if width != U::ZERO && abs_bits >= width {
            -T::from(x < T::ZERO)
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shr_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// non-negative amount; shifting by a large amount returns `Some(0)` if `self`
                    /// is positive, and `Some(-1)` if `self` is negative.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(\lfloor x/2^b \rfloor) &
                    ///         \text{if} \\quad b \geq 0, \\\\
                    ///     \operatorname{Some}(2^{-b} x) &
                    ///         \text{if} \\quad b < 0 \\ \mathrm{and}
                    ///         \\ -2^{W-1} \leq 2^{-b} x < 2^{W-1}, \\\\
                    ///     \operatorname{None} &
                    ///         \text{if} \\quad b < 0 \\ \mathrm{and}
                    ///         \\ (2^{-b} x < -2^{W-1} \\ \mathrm{or}
                    ///         \\ 2^{-b} x \geq 2^{W-1}), \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shr#arithmetic_checked_shr).
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shr_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed);
