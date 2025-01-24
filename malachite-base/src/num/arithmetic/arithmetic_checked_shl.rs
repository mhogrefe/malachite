// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use core::ops::{Shl, Shr};

fn arithmetic_checked_shl_unsigned_unsigned<
    T: PrimitiveUnsigned + Shl<U, Output = T> + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64>,
>(
    x: T,
    bits: U,
) -> Option<T> {
    if x == T::ZERO {
        Some(x)
    } else if bits >= U::wrapping_from(T::WIDTH) {
        None
    } else {
        let result = x << bits;
        if result >> bits == x {
            Some(result)
        } else {
            None
        }
    }
}

macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) & \text{if} \\quad 2^b x < 2^W, \\\\
                    ///     \operatorname{None} & \text{if} \\quad 2^b x \geq 2^W,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shl#arithmetic_checked_shl).
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shl_unsigned_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned);

fn arithmetic_checked_shl_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveUnsigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits >= S::ZERO {
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

macro_rules! impl_arithmetic_checked_shl_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// negative amount; shifting by a negative amount with a high absolute value
                    /// returns `Some(0)`.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) &
                    ///         \text{if} \\quad b \geq 0 \\ \mathrm{and}\\ 2^b x < 2^W, \\\\
                    ///     \operatorname{None} &
                    ///         \text{if} \\quad b \geq 0 \\ \mathrm{and} \\ 2^b x \geq 2^W, \\\\
                    ///     \operatorname{Some}(\lfloor x/2^{-b} \rfloor) &
                    ///         \text{if} \\quad b < 0,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shl#arithmetic_checked_shl).
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shl_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_signed);

fn arithmetic_checked_shl_signed_unsigned<
    U: ArithmeticCheckedShl<B, Output = U> + PrimitiveUnsigned,
    S: TryFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    B,
>(
    x: S,
    bits: B,
) -> Option<S> {
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        abs.arithmetic_checked_shl(bits)
            .and_then(|x| S::try_from(x).ok())
    } else {
        abs.arithmetic_checked_shl(bits).and_then(|x| {
            if x == S::MIN.unsigned_abs() {
                Some(S::MIN)
            } else {
                S::try_from(x).ok().map(|y| -y)
            }
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) &
                    ///         \text{if} \\quad -2^{W-1} \leq 2^b x < 2^{W-1}, \\\\
                    ///     \operatorname{None} &
                    ///         \text{if} \\quad 2^b x < -2^{W-1} \\ \mathrm{or}
                    ///         \\ 2^b x \geq 2^{W-1}, \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shl#arithmetic_checked_shl).
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shl_signed_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_unsigned);

fn arithmetic_checked_shl_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveSigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits >= S::ZERO {
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

macro_rules! impl_arithmetic_checked_shl_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2). If the result is too
                    /// large to be represented, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// negative amount; shifting by a negative amount with a high absolute value
                    /// returns `Some(0)` if `self` is positive, and `Some(-1)` if `self` is
                    /// negative.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) &
                    ///         \text{if} \\quad b \geq 0 \\ \mathrm{and}
                    ///         \\ -2^{W-1} \leq 2^b x < 2^{W-1}, \\\\
                    ///     \operatorname{None} &
                    ///         \text{if} \\quad b \geq 0 \\ \mathrm{and}
                    ///         \\ (2^b x < -2^{W-1} \\ \mathrm{or} \\ 2^b x \geq 2^{W-1}), \\\\
                    ///     \operatorname{Some}(\lfloor x/2^{-b} \rfloor) & \text{if} \\quad b < 0,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `Self::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::arithmetic_checked_shl#arithmetic_checked_shl).
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        arithmetic_checked_shl_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed);
