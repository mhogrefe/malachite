// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ShrRoundAssign, UnsignedAbs};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::mantissa_and_exponent::sci_mantissa_and_exponent_round;
use crate::num::conversion::traits::{
    ConvertibleFrom, OverflowingFrom, RoundingFrom, SaturatingFrom, SciMantissaAndExponent,
    WrappingFrom,
};
use crate::num::float::NiceFloat;
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};
use core::ops::Neg;

// This macro defines conversions from a type to itself.
macro_rules! identity_conversion {
    ($t:ty) => {
        impl WrappingFrom<$t> for $t {
            /// Converts a value to its own type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#wrapping_from).
            #[inline]
            fn wrapping_from(value: $t) -> $t {
                value
            }
        }

        impl SaturatingFrom<$t> for $t {
            /// Converts a value to its own type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#saturating_from).
            #[inline]
            fn saturating_from(value: $t) -> $t {
                value
            }
        }

        impl OverflowingFrom<$t> for $t {
            /// Converts a value to its own type. Since this conversion is always valid and always
            /// leaves the value unchanged, the second component of the result is always false (no
            /// overflow).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#overflowing_from).
            #[inline]
            fn overflowing_from(value: $t) -> ($t, bool) {
                (value, false)
            }
        }

        impl ConvertibleFrom<$t> for $t {
            /// Checks whether a value is convertible to its own type. The result is always `true`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#convertible_from).
            #[inline]
            fn convertible_from(_: $t) -> bool {
                true
            }
        }
    };
}

// This macro defines conversions from type $a to type $b, where every value of type $a is
// representable by a value of type $b.
macro_rules! lossless_conversion {
    ($a:ty, $b:ident) => {
        impl WrappingFrom<$a> for $b {
            /// Converts a value to another type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#wrapping_from).
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        impl SaturatingFrom<$a> for $b {
            /// Converts a value to another type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#saturating_from).
            #[inline]
            fn saturating_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        impl OverflowingFrom<$a> for $b {
            /// Converts a value to the value's type. Since this conversion is always valid and
            /// always leaves the value unchanged, the second component of the result is always
            /// false (no overflow).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#overflowing_from).
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                ($b::from(value), false)
            }
        }

        impl ConvertibleFrom<$a> for $b {
            /// Checks whether a value is convertible to a different type. The result is always
            /// `true`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#convertible_from).
            #[inline]
            fn convertible_from(_: $a) -> bool {
                true
            }
        }
    };
}

fn saturating_from_lossy<A: TryFrom<B> + PrimitiveInt, B: PrimitiveInt + WrappingFrom<A>>(
    value: A,
) -> B {
    if let Ok(b_max) = A::try_from(B::MAX) {
        if value >= b_max {
            return B::MAX;
        }
    }
    if let Ok(b_min) = A::try_from(B::MIN) {
        if value <= b_min {
            return B::MIN;
        }
    }
    B::wrapping_from(value)
}

fn overflowing_from_lossy<A: PrimitiveInt + WrappingFrom<B>, B: PrimitiveInt + WrappingFrom<A>>(
    value: A,
) -> (B, bool) {
    let result = B::wrapping_from(value);
    (
        result,
        (result >= B::ZERO) != (value >= A::ZERO) || A::wrapping_from(result) != value,
    )
}

fn convertible_from_lossy<A: PrimitiveInt + WrappingFrom<B>, B: PrimitiveInt + WrappingFrom<A>>(
    value: A,
) -> bool {
    let result = B::wrapping_from(value);
    (result >= B::ZERO) == (value >= A::ZERO) && A::wrapping_from(result) == value
}

// This macro defines conversions from type $a to type $b, where not every value of type $a is
// representable by a value of type $b.
macro_rules! lossy_conversion {
    ($a:ident, $b:ident) => {
        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$a> for $b {
            /// Converts a value to another type. If the value cannot be represented in the new
            /// type, it is wrapped.
            ///
            /// Let $W$ be the width of the target type.
            ///
            /// If the target type is unsigned, then $f_W(n) = m$, where $m < 2^W$ and $n + 2^W k =
            /// m$ for some $k \in \Z$.
            ///
            /// If the target type is signed, then $f_W(n) = m$, where $-2^{W-1} \leq m < 2^{W-1}$
            /// and $n + 2^W k = m$ for some $k \in \Z$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#wrapping_from).
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                value as $b
            }
        }

        impl SaturatingFrom<$a> for $b {
            /// Converts a value to another type. If the value cannot be represented in the new
            /// type, the maximum or minimum value of the new type, whichever is closer, is
            /// returned.
            ///
            /// Let $W$ be the width of the target type.
            ///
            /// If the target type is unsigned, then
            /// $$
            /// f_W(n) = \\begin{cases}
            ///     0 & n < 0 \\\\
            ///     2^W-1 & \text{if} \\quad n \geq 2^W, \\\\
            ///     n & \\text{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// If the target type is signed, then
            /// $$
            /// f_W(n) = \\begin{cases}
            ///     -2^{W-1} & \text{if} \\quad n < -2^{W-1}, \\\\
            ///     2^{W-1}-1 & \text{if} \\quad n \geq 2^{W-1}, \\\\
            ///     n & \\text{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#saturating_from).
            #[inline]
            fn saturating_from(value: $a) -> $b {
                saturating_from_lossy(value)
            }
        }

        impl OverflowingFrom<$a> for $b {
            /// Converts a value to another type. If the value cannot be represented in the new
            /// type, it is wrapped. The second component of the result indicates whether overflow
            /// occurred.
            ///
            /// Let $W$ be the width of the target type.
            ///
            /// If the target type is unsigned, then $f_W(n) = (m, k \neq 0)$, where $m < 2^W$ and
            /// $n + 2^W k = m$ for some $k \in \Z$.
            ///
            /// If the target type is signed, then $f_W(n) = (m, k \neq 0)$, where $-2^{W-1} \leq m
            /// < 2^{W-1}$ and $n + 2^W k = m$ for some $k \in \Z$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#overflowing_from).
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                overflowing_from_lossy(value)
            }
        }

        impl ConvertibleFrom<$a> for $b {
            /// Determines whether a value is convertible to a different type.
            ///
            /// Let $W$ be the width of the target type.
            ///
            /// If the target type is unsigned then,
            /// $$
            /// f_W(n) = (0 \leq n < 2^W).
            /// $$
            ///
            /// If the target type is signed then,
            /// $$
            /// f_W(n) = (-2^{W-1} \leq n < 2^{W-1}-1).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from#convertible_from).
            #[inline]
            fn convertible_from(value: $a) -> bool {
                convertible_from_lossy::<$a, $b>(value)
            }
        }
    };
}

// This macro defines conversions from type $a to type $b, where the set of values representable by
// type $a is a proper subset of the set of values representable by type $b.
macro_rules! proper_subset_conversion {
    ($a:ident, $b:ident) => {
        lossless_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

// This macro defines conversions from type $a to type $b, where the set of values representable by
// type $a is neither a subset nor a superset of the set of values representable by type $b.
macro_rules! no_containment_conversion {
    ($a:ident, $b:ident) => {
        lossy_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

apply_to_primitive_ints!(identity_conversion);

proper_subset_conversion!(u8, u16);
proper_subset_conversion!(u8, u32);
proper_subset_conversion!(u8, u64);
proper_subset_conversion!(u8, u128);
proper_subset_conversion!(u8, usize);
proper_subset_conversion!(u8, i16);
proper_subset_conversion!(u8, i32);
proper_subset_conversion!(u8, i64);
proper_subset_conversion!(u8, i128);
proper_subset_conversion!(u8, isize);
proper_subset_conversion!(u16, u32);
proper_subset_conversion!(u16, u64);
proper_subset_conversion!(u16, u128);
proper_subset_conversion!(u16, usize);
proper_subset_conversion!(u16, i32);
proper_subset_conversion!(u16, i64);
proper_subset_conversion!(u16, i128);
proper_subset_conversion!(u32, u64);
proper_subset_conversion!(u32, u128);
proper_subset_conversion!(u32, i64);
proper_subset_conversion!(u32, i128);
proper_subset_conversion!(u64, u128);
proper_subset_conversion!(u64, i128);
proper_subset_conversion!(i8, i16);
proper_subset_conversion!(i8, i32);
proper_subset_conversion!(i8, i64);
proper_subset_conversion!(i8, i128);
proper_subset_conversion!(i8, isize);
proper_subset_conversion!(i16, i32);
proper_subset_conversion!(i16, i64);
proper_subset_conversion!(i16, i128);
proper_subset_conversion!(i16, isize);
proper_subset_conversion!(i32, i64);
proper_subset_conversion!(i32, i128);
proper_subset_conversion!(i64, i128);

no_containment_conversion!(u8, i8);
no_containment_conversion!(u16, i8);
no_containment_conversion!(u16, i16);
no_containment_conversion!(u16, isize);
no_containment_conversion!(u32, usize);
no_containment_conversion!(u32, i8);
no_containment_conversion!(u32, i16);
no_containment_conversion!(u32, i32);
no_containment_conversion!(u32, isize);
no_containment_conversion!(u64, usize);
no_containment_conversion!(u64, i8);
no_containment_conversion!(u64, i16);
no_containment_conversion!(u64, i32);
no_containment_conversion!(u64, i64);
no_containment_conversion!(u64, isize);
no_containment_conversion!(u128, usize);
no_containment_conversion!(u128, i8);
no_containment_conversion!(u128, i16);
no_containment_conversion!(u128, i32);
no_containment_conversion!(u128, i64);
no_containment_conversion!(u128, i128);
no_containment_conversion!(u128, isize);
no_containment_conversion!(usize, i8);
no_containment_conversion!(usize, i16);
no_containment_conversion!(usize, i32);
no_containment_conversion!(usize, i64);
no_containment_conversion!(usize, i128);
no_containment_conversion!(usize, isize);
no_containment_conversion!(i32, isize);
no_containment_conversion!(i64, isize);
no_containment_conversion!(i128, isize);

fn primitive_float_rounding_from_unsigned<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    value: U,
    rm: RoundingMode,
) -> (T, Ordering) {
    if value == U::ZERO {
        return (T::ZERO, Equal);
    }
    let (mantissa, exponent, o) = sci_mantissa_and_exponent_round(value, rm).unwrap();
    if let Some(f) = T::from_sci_mantissa_and_exponent(mantissa, i64::wrapping_from(exponent)) {
        (f, o)
    } else {
        match rm {
            Exact => {
                panic!("Value cannot be represented exactly as an {}", T::NAME)
            }
            Floor | Down | Nearest => (T::MAX_FINITE, Less),
            _ => (T::INFINITY, Greater),
        }
    }
}

fn unsigned_rounding_from_primitive_float<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    value: U,
    rm: RoundingMode,
) -> (T, Ordering) {
    assert!(!value.is_nan());
    if value.is_infinite() {
        return if value.is_sign_positive() {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as a {}", T::NAME)
                }
                Down | Floor | Nearest => (T::MAX, Less),
                _ => panic!("Cannot round away from positive infinity"),
            }
        } else {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as a {}", T::NAME)
                }
                Down | Ceiling | Nearest => (T::ZERO, Greater),
                _ => panic!("Cannot round away from negative infinity"),
            }
        };
    }
    if value == U::ZERO {
        return (T::ZERO, Equal);
    }
    if value.is_sign_negative() {
        return match rm {
            Exact => {
                panic!("Value cannot be represented exactly as a {}", T::NAME)
            }
            Ceiling | Down | Nearest => (T::ZERO, Greater),
            _ => panic!("Value is less than 0 and rounding mode is {rm}"),
        };
    }
    let (mut mantissa, exponent) = value.integer_mantissa_and_exponent();
    let (result, o) = if exponent <= 0 {
        let o = mantissa.shr_round_assign(-exponent, rm);
        (T::try_from(mantissa).ok(), o)
    } else {
        (
            T::try_from(mantissa)
                .ok()
                .and_then(|n| n.arithmetic_checked_shl(exponent)),
            Equal,
        )
    };
    if let Some(n) = result {
        (n, o)
    } else {
        match rm {
            Exact => {
                panic!("Value cannot be represented exactly as a {}", T::NAME)
            }
            Floor | Down | Nearest => (T::MAX, Less),
            _ => panic!(
                "Value is greater than {}::MAX and rounding mode is {}",
                T::NAME,
                rm
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveFloatFromUnsignedError;

fn primitive_float_try_from_unsigned<T: PrimitiveFloat, U: PrimitiveUnsigned>(
    value: U,
) -> Result<T, PrimitiveFloatFromUnsignedError> {
    if value == U::ZERO {
        return Ok(T::ZERO);
    }
    let (mantissa, exponent, _) =
        sci_mantissa_and_exponent_round(value, Exact).ok_or(PrimitiveFloatFromUnsignedError)?;
    T::from_sci_mantissa_and_exponent(mantissa, i64::wrapping_from(exponent))
        .ok_or(PrimitiveFloatFromUnsignedError)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsignedFromFloatError {
    FloatInfiniteOrNan,
    FloatNegative,
    FloatNonIntegerOrOutOfRange,
}

fn unsigned_try_from_primitive_float<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    value: U,
) -> Result<T, UnsignedFromFloatError> {
    if !value.is_finite() {
        Err(UnsignedFromFloatError::FloatInfiniteOrNan)
    } else if value == U::ZERO {
        Ok(T::ZERO)
    } else if value < U::ZERO {
        Err(UnsignedFromFloatError::FloatNegative)
    } else {
        let (mantissa, exponent) = value.integer_mantissa_and_exponent();
        if exponent < 0 {
            Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
        } else {
            T::try_from(mantissa)
                .or(Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange))
                .and_then(|n| {
                    n.arithmetic_checked_shl(exponent)
                        .ok_or(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                })
        }
    }
}

fn primitive_float_convertible_from_unsigned<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned + SciMantissaAndExponent<T, u64>,
>(
    value: U,
) -> bool {
    if value == U::ZERO {
        return true;
    }
    let precision = (value >> value.trailing_zeros()).significant_bits();
    precision <= T::MANTISSA_WIDTH + 1
        && i64::wrapping_from(SciMantissaAndExponent::<T, u64>::sci_exponent(value))
            <= T::MAX_EXPONENT
}

#[inline]
fn unsigned_convertible_from_primitive_float<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    value: U,
) -> bool {
    value >= U::ZERO
        && value.is_integer()
        && (value == U::ZERO || value.sci_exponent() < i64::wrapping_from(T::WIDTH))
}

macro_rules! impl_from_float_unsigned {
    ($u:ident) => {
        macro_rules! impl_from_float_unsigned_inner {
            ($f:ident) => {
                impl RoundingFrom<$u> for $f {
                    /// Converts a value of an unsigned type to a value of a floating point type
                    /// according to a specified [`RoundingMode`]. An [`Ordering`] is also returned,
                    /// indicating whether the returned value is less than, equal to, or greater
                    /// than the original value.
                    ///
                    /// - If the rounding mode is `Floor` or `Down`, the largest float less than or
                    ///   equal to the value is returned.
                    /// - If the rounding mode is `Ceiling` or `Up`, the smallest float greater than
                    ///   or equal to the value is returned.
                    /// - If the rounding mode is `Nearest`, then the nearest float is returned. If
                    ///   the value is exactly between two floats, the float with the zero
                    ///   least-significant bit in its representation is selected. If the value is
                    ///   larger than the maximum finite float (which can only happen when
                    ///   converting a `u128` to an `f32`), the maximum finite float is returned.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `value` is not exactly equal to any value of
                    /// the primitive float type.
                    ///
                    /// # Examples
                    /// See [here](super::from#rounding_from).
                    #[inline]
                    fn rounding_from(value: $u, rm: RoundingMode) -> ($f, Ordering) {
                        primitive_float_rounding_from_unsigned(value, rm)
                    }
                }

                impl RoundingFrom<$f> for $u {
                    /// Converts a value of a floating point type to a value of an unsigned type
                    /// according to a specified [`RoundingMode`]. An [`Ordering`] is also returned,
                    /// indicating whether the returned value is less than, equal to, or greater
                    /// than the original value.
                    ///
                    /// - If the rounding mode is `Floor`, the largest number less than or equal to
                    ///   the value is returned. If the float is greater than the maximum
                    ///   representable unsigned value, the maximum unsigned value is returned. If
                    ///   the float is negative, the function panics.
                    /// - If the rounding mode is `Ceiling`, the smallest number greater than or
                    ///   equal to the value is returned. If the float is negative, zero is
                    ///   returned. If the float is greater than the maximum representable unsigned
                    ///   value, the function panics.
                    /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor`
                    ///   if the float is non-negative and as with `Ceiling` if the value is
                    ///   negative.
                    /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling`
                    ///   if the value is non-negative and as with `Floor` if the value is negative.
                    /// - If the rounding mode is `Nearest`, then the nearest value is returned. If
                    ///   the value is exactly between two numbers, the even one is selected. If the
                    ///   float is greater than the maximum representable unsigned value, the
                    ///   maximum unsigned value is returned. If the float is negative, zero is
                    ///   returned.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// - If `value` is `NaN`.
                    /// - If `rm` is `Exact` but `value` is not exactly equal to any value of the
                    ///   unsigned type.
                    /// - If `value` is greater than the maximum value of the unsigned type and `rm`
                    ///   is `Ceiling` or `Up`.
                    /// - If `value` is negative and `rm` is `Floor` or `Up`.
                    ///
                    /// # Examples
                    /// See [here](super::from#rounding_from).
                    #[inline]
                    fn rounding_from(value: $f, rm: RoundingMode) -> ($u, Ordering) {
                        unsigned_rounding_from_primitive_float(value, rm)
                    }
                }

                impl TryFrom<$u> for NiceFloat<$f> {
                    type Error = PrimitiveFloatFromUnsignedError;

                    /// Converts a value of an unsigned type to a value of a floating point type,
                    /// returning an error if an exact conversion is not possible.
                    ///
                    /// The conversion succeeds if the unsigned value is not too large to represent
                    /// (which can only happen when converting a [`u128`] to an [`f32`]) and the
                    /// precision of the unsigned value is not too high.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#try_from).
                    #[inline]
                    fn try_from(value: $u) -> Result<NiceFloat<$f>, Self::Error> {
                        primitive_float_try_from_unsigned(value).map(NiceFloat)
                    }
                }

                impl TryFrom<NiceFloat<$f>> for $u {
                    type Error = UnsignedFromFloatError;

                    /// Converts a value of a floating point type to a value of an unsigned type,
                    /// returning an error if an exact conversion is not possible.
                    ///
                    /// The conversion succeeds if the floating point value is an integer, not
                    /// negative (negative zero is ok), and not too large.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#try_from).
                    #[inline]
                    fn try_from(value: NiceFloat<$f>) -> Result<$u, Self::Error> {
                        unsigned_try_from_primitive_float(value.0)
                    }
                }

                impl ConvertibleFrom<$u> for $f {
                    /// Checks whether a value of an unsigned type is convertible to a floating
                    /// point type.
                    ///
                    /// An exact conversion is possible if the unsigned value is not too large to
                    /// represent (which can only happen when converting a [`u128`] to an [`f32`])
                    /// and the precision of the unsigned value is not too high.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#convertible_from).
                    #[inline]
                    fn convertible_from(value: $u) -> bool {
                        primitive_float_convertible_from_unsigned::<$f, $u>(value)
                    }
                }

                impl ConvertibleFrom<$f> for $u {
                    /// Checks whether a value of a floating point type is convertible to an
                    /// unsigned type.
                    ///
                    /// An exact conversion is possible if the floating point value is an integer,
                    /// not negative (negative zero is ok), and not too large.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#convertible_from).
                    #[inline]
                    fn convertible_from(value: $f) -> bool {
                        unsigned_convertible_from_primitive_float::<$u, $f>(value)
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_from_float_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_from_float_unsigned);

#[inline]
fn primitive_float_rounding_from_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
    F: PrimitiveFloat + RoundingFrom<U>,
>(
    value: S,
    rm: RoundingMode,
) -> (F, Ordering) {
    let abs = value.unsigned_abs();
    if value >= S::ZERO {
        F::rounding_from(abs, rm)
    } else {
        let (x, o) = F::rounding_from(abs, -rm);
        (-x, o.reverse())
    }
}

fn signed_rounding_from_primitive_float<
    U: PrimitiveUnsigned + RoundingFrom<F>,
    S: TryFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    F: PrimitiveFloat,
>(
    value: F,
    rm: RoundingMode,
) -> (S, Ordering) {
    if value.is_infinite() {
        return if value.is_sign_positive() {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as a {}", S::NAME)
                }
                Down | Floor | Nearest => (S::MAX, Less),
                _ => panic!("Cannot round away from extreme value"),
            }
        } else {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as a {}", S::NAME)
                }
                Down | Nearest | Ceiling => (S::MIN, Greater),
                _ => panic!("Cannot round away from extreme value"),
            }
        };
    }
    if value == F::ZERO {
        return (S::ZERO, Equal);
    }
    if value.is_sign_positive() {
        let (abs, o) = U::rounding_from(value, rm);
        if let Ok(n) = S::try_from(abs) {
            (n, o)
        } else {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as an {}", S::NAME)
                }
                Floor | Down | Nearest => (S::MAX, Less),
                _ => panic!(
                    "Value is greater than {}::MAX and rounding mode is {}",
                    S::NAME,
                    rm
                ),
            }
        }
    } else {
        let (abs, o) = U::rounding_from(-value, -rm);
        let n = if abs == S::MIN.unsigned_abs() {
            Some(S::MIN)
        } else {
            S::try_from(abs).ok().map(Neg::neg)
        };
        if let Some(n) = n {
            (n, o.reverse())
        } else {
            match rm {
                Exact => {
                    panic!("Value cannot be represented exactly as an {}", S::NAME)
                }
                Ceiling | Down | Nearest => (S::MIN, Greater),
                _ => panic!(
                    "Value is smaller than {}::MIN and rounding mode is {}",
                    S::NAME,
                    rm
                ),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveFloatFromSignedError;

#[inline]
fn primitive_float_try_from_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
    F: PrimitiveFloat,
>(
    value: S,
) -> Result<F, PrimitiveFloatFromSignedError>
where
    NiceFloat<F>: TryFrom<U>,
{
    let abs = value.unsigned_abs();
    if value >= S::ZERO {
        NiceFloat::<F>::try_from(abs)
            .map(|f| f.0)
            .map_err(|_| PrimitiveFloatFromSignedError)
    } else {
        NiceFloat::<F>::try_from(abs)
            .map(|f| -f.0)
            .map_err(|_| PrimitiveFloatFromSignedError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignedFromFloatError {
    FloatInfiniteOrNan,
    FloatNonIntegerOrOutOfRange,
}

fn signed_try_from_primitive_float<
    U: TryFrom<NiceFloat<F>> + PrimitiveUnsigned,
    S: TryFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    F: PrimitiveFloat,
>(
    value: F,
) -> Result<S, SignedFromFloatError> {
    if !value.is_finite() {
        return Err(SignedFromFloatError::FloatInfiniteOrNan);
    }
    if value >= F::ZERO {
        S::try_from(
            U::try_from(NiceFloat(value))
                .or(Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange))?,
        )
        .or(Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange))
    } else {
        let abs = U::try_from(NiceFloat(-value))
            .or(Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange))?;
        if abs == S::MIN.unsigned_abs() {
            Ok(S::MIN)
        } else {
            S::try_from(abs)
                .map(Neg::neg)
                .or(Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange))
        }
    }
}

#[inline]
fn primitive_float_convertible_from_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
    F: ConvertibleFrom<U> + PrimitiveFloat,
>(
    value: S,
) -> bool {
    F::convertible_from(value.unsigned_abs())
}

fn signed_convertible_from_primitive_float<U: PrimitiveUnsigned, F: PrimitiveFloat>(
    value: F,
) -> bool {
    if !value.is_integer() {
        return false;
    }
    if value >= F::ZERO {
        value == F::ZERO || value.sci_exponent() < i64::wrapping_from(U::WIDTH) - 1
    } else {
        let exponent = value.sci_exponent();
        let limit = i64::wrapping_from(U::WIDTH) - 1;
        value == F::ZERO
            || exponent < limit
            || exponent == limit
                && value == -F::from_sci_mantissa_and_exponent(F::ONE, exponent).unwrap()
    }
}

macro_rules! impl_from_float_signed {
    ($u:ident, $i:ident) => {
        macro_rules! impl_from_float_signed_inner {
            ($f:ident) => {
                impl RoundingFrom<$i> for $f {
                    /// Converts a value of a signed type to a value of a floating point type
                    /// according to a specified [`RoundingMode`]. An [`Ordering`] is also returned,
                    /// indicating whether the returned value is less than, equal to, or greater
                    /// than the original value.
                    ///
                    /// - If the rounding mode is `Floor`, the largest float less than or equal to
                    ///   the value is returned.
                    /// - If the rounding mode is `Ceiling`, the smallest float greater than or
                    ///   equal to the value is returned.
                    /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor`
                    ///   if the value is non-negative and as with `Ceiling` if the value is
                    ///   negative.
                    /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling`
                    ///   if the value is non-negative and as with `Floor` if the value is negative.
                    /// - If the rounding mode is `Nearest`, then the nearest float is returned. If
                    ///   the value is exactly between two floats, the float with the zero
                    ///   least-significant bit in its representation is selected.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `Exact` but `value` is not exactly equal to any value of
                    /// the primitive float type.
                    ///
                    /// # Examples
                    /// See [here](super::from#rounding_from).
                    #[inline]
                    fn rounding_from(value: $i, rm: RoundingMode) -> ($f, Ordering) {
                        primitive_float_rounding_from_signed::<$u, $i, $f>(value, rm)
                    }
                }

                impl RoundingFrom<$f> for $i {
                    /// Converts a value of a floating point type to a value of a signed type
                    /// according to a specified [`RoundingMode`]. An [`Ordering`] is also returned,
                    /// indicating whether the returned value is less than, equal to, or greater
                    /// than the original value.
                    ///
                    /// - If the rounding mode is `Floor`, the largest number less than or equal to
                    ///   the value is returned. If the float is greater than the maximum
                    ///   representable signed value, the maximum signed value is returned. If the
                    ///   float is smaller than the minimum representable signed value, the function
                    ///   panics.
                    /// - If the rounding mode is `Ceiling`, the smallest number greater than or
                    ///   equal to the value is returned. If the float is smaller than the minimum
                    ///   representable signed value, the minimum signed value is returned. If the
                    ///   float is greater than the maximum representable signed value, the function
                    ///   panics.
                    /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor`
                    ///   if the float is non-negative and as with `Ceiling` if the value is
                    ///   negative.
                    /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling`
                    ///   if the value is non-negative and as with `Floor` if the value is negative.
                    /// - If the rounding mode is `Nearest`, then the nearest value is returned. If
                    ///   the value is exactly between two numbers, the even one is selected. If the
                    ///   float is greater than the maximum representable signed value, the maximum
                    ///   signed value is returned. If the float is smaller than the minimum
                    ///   representable signed value, the minimum signed value is returned.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// - If `value` is `NaN`.
                    /// - If `rm` is `Exact` but `value` is not exactly equal to any value of the
                    ///   unsigned type.
                    /// - If `value` is greater than the maximum value of the signed type and `rm`
                    ///   is `Ceiling` or `Up`.
                    /// - If `value` is smaller than the minimum value of the signed type and `rm`
                    ///   is `Floor` or `Up`.
                    ///
                    /// # Examples
                    /// See [here](super::from#rounding_from).
                    #[inline]
                    fn rounding_from(value: $f, rm: RoundingMode) -> ($i, Ordering) {
                        signed_rounding_from_primitive_float::<$u, $i, $f>(value, rm)
                    }
                }

                impl TryFrom<$i> for NiceFloat<$f> {
                    type Error = PrimitiveFloatFromSignedError;

                    /// Converts a value of a signed type to a value of a floating point type,
                    /// returning an error if an exact conversion is not possible.
                    ///
                    /// The conversion succeeds if the precision of the signed value is not too
                    /// high.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#try_from).
                    #[inline]
                    fn try_from(value: $i) -> Result<NiceFloat<$f>, Self::Error> {
                        primitive_float_try_from_signed(value).map(NiceFloat)
                    }
                }

                impl TryFrom<NiceFloat<$f>> for $i {
                    type Error = SignedFromFloatError;

                    /// Converts a value of a floating point type to a value of a signed type,
                    /// returning an error if an exact conversion is not possible.
                    ///
                    /// The conversion succeeds if the floating point value is an integer and not
                    /// too large or too small.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#try_from).
                    #[inline]
                    fn try_from(value: NiceFloat<$f>) -> Result<$i, Self::Error> {
                        signed_try_from_primitive_float::<$u, $i, $f>(value.0)
                    }
                }

                impl ConvertibleFrom<$i> for $f {
                    /// Checks whether a value of a signed type is convertible to a floating point
                    /// type.
                    ///
                    /// An exact conversion is possible if the precision of the signed value is not
                    /// too high.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#convertible_from).
                    #[inline]
                    fn convertible_from(value: $i) -> bool {
                        primitive_float_convertible_from_signed::<$u, $i, $f>(value)
                    }
                }

                impl ConvertibleFrom<$f> for $i {
                    /// Checks whether a value of a floating point type is convertible to a signed
                    /// type.
                    ///
                    /// An exact conversion is possible if the floating point value is an integer
                    /// and not too large or too small.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::from#convertible_from).
                    #[inline]
                    fn convertible_from(value: $f) -> bool {
                        signed_convertible_from_primitive_float::<$u, $f>(value)
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_from_float_signed_inner);
    };
}
apply_to_unsigned_signed_pairs!(impl_from_float_signed);
