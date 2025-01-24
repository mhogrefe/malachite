// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivisibleByPowerOf2, ModPowerOf2, ShrRound,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
    WrappingFrom,
};
use crate::num::logic::traits::{BitAccess, LeadingZeros, LowMask, SignificantBits, TrailingZeros};
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};

fn raw_mantissa_and_exponent<T: PrimitiveFloat>(x: T) -> (u64, u64) {
    let bits = x.to_bits();
    (
        bits.mod_power_of_2(T::MANTISSA_WIDTH),
        (bits >> T::MANTISSA_WIDTH).mod_power_of_2(T::EXPONENT_WIDTH),
    )
}

#[inline]
fn raw_mantissa<T: PrimitiveFloat>(x: T) -> u64 {
    x.to_bits().mod_power_of_2(T::MANTISSA_WIDTH)
}

#[inline]
fn raw_exponent<T: PrimitiveFloat>(x: T) -> u64 {
    (x.to_bits() >> T::MANTISSA_WIDTH).mod_power_of_2(T::EXPONENT_WIDTH)
}

fn from_raw_mantissa_and_exponent<T: PrimitiveFloat>(raw_mantissa: u64, raw_exponent: u64) -> T {
    assert!(raw_mantissa.significant_bits() <= T::MANTISSA_WIDTH);
    assert!(raw_exponent.significant_bits() <= T::EXPONENT_WIDTH);
    let x = T::from_bits((raw_exponent << T::MANTISSA_WIDTH) | raw_mantissa);
    // Only output the canonical NaN
    if x.is_nan() {
        T::NAN
    } else {
        x
    }
}

fn integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> (u64, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        let trailing_zeros = raw_mantissa.trailing_zeros();
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(trailing_zeros) + T::MIN_EXPONENT,
        )
    } else {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
        let trailing_zeros = TrailingZeros::trailing_zeros(raw_mantissa);
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(raw_exponent + trailing_zeros) + T::MIN_EXPONENT - 1,
        )
    }
}

fn integer_mantissa_primitive_float<T: PrimitiveFloat>(x: T) -> u64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent != 0 {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
    }
    raw_mantissa >> raw_mantissa.trailing_zeros()
}

fn integer_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        i64::wrapping_from(raw_mantissa.trailing_zeros()) + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(
            raw_exponent
                + if raw_mantissa == 0 {
                    T::MANTISSA_WIDTH
                } else {
                    TrailingZeros::trailing_zeros(raw_mantissa)
                },
        ) + T::MIN_EXPONENT
            - 1
    }
}

fn from_integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    integer_mantissa: u64,
    integer_exponent: i64,
) -> Option<T> {
    if integer_mantissa == 0 {
        return Some(T::ZERO);
    }
    let trailing_zeros = integer_mantissa.trailing_zeros();
    let (integer_mantissa, adjusted_exponent) = (
        integer_mantissa >> trailing_zeros,
        integer_exponent + i64::wrapping_from(trailing_zeros),
    );
    let mantissa_bits = integer_mantissa.significant_bits();
    let sci_exponent = adjusted_exponent.checked_add(i64::exact_from(mantissa_bits))? - 1;
    let mut raw_mantissa;
    let raw_exponent;
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    } else if sci_exponent < T::MIN_NORMAL_EXPONENT {
        if adjusted_exponent < T::MIN_EXPONENT {
            return None;
        }
        raw_exponent = 0;
        raw_mantissa = integer_mantissa << (adjusted_exponent - T::MIN_EXPONENT);
    } else if mantissa_bits > T::MANTISSA_WIDTH + 1 {
        return None;
    } else {
        raw_exponent = u64::exact_from(sci_exponent + i64::low_mask(T::EXPONENT_WIDTH - 1));
        raw_mantissa = integer_mantissa << (T::MANTISSA_WIDTH + 1 - mantissa_bits);
        raw_mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    Some(T::from_raw_mantissa_and_exponent(
        raw_mantissa,
        raw_exponent,
    ))
}

fn sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> (T, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        let leading_zeros =
            LeadingZeros::leading_zeros(raw_mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH);
        let mut mantissa = raw_mantissa << (leading_zeros + 1);
        mantissa.clear_bit(T::MANTISSA_WIDTH);
        (
            T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(T::MANTISSA_WIDTH - leading_zeros - 1) + T::MIN_EXPONENT,
        )
    } else {
        (
            T::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT,
        )
    }
}

fn sci_mantissa_primitive_float<T: PrimitiveFloat>(x: T) -> T {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        mantissa <<= LeadingZeros::leading_zeros(mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH) + 1;
        mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT))
}

fn sci_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(raw_mantissa) - 1)
            + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT
    }
}

#[allow(clippy::wrong_self_convention)]
fn from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    sci_mantissa: T,
    sci_exponent: i64,
) -> Option<T> {
    assert!(sci_mantissa.is_finite());
    assert!(sci_mantissa > T::ZERO);
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    }
    let (mut orig_mantissa, orig_exponent) = sci_mantissa.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if orig_exponent != u64::wrapping_from(T::MAX_EXPONENT) {
        return None;
    }
    if sci_exponent < T::MIN_NORMAL_EXPONENT {
        let shift = T::MIN_NORMAL_EXPONENT - sci_exponent;
        if orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(shift)) {
            orig_mantissa.set_bit(T::MANTISSA_WIDTH);
            Some(T::from_raw_mantissa_and_exponent(orig_mantissa >> shift, 0))
        } else {
            None
        }
    } else {
        Some(T::from_raw_mantissa_and_exponent(
            orig_mantissa,
            u64::wrapping_from(sci_exponent + T::MAX_EXPONENT),
        ))
    }
}

/// Returns the scientific mantissa and exponent of an unsinged value. An [`Ordering`] is also
/// returned, indicating whether the mantissa and exponent correspond to a value less than, equal
/// to, or greater than the original value.
///
/// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is a
/// rational number with $1 \leq m_s < 2$. We represent the rational mantissa as a float. The
/// conversion might not be exact, so we round to the nearest float using the provided rounding
/// mode. If the rounding mode is `Exact` but the conversion is not exact, `None` is returned.
/// $$
/// f(x, r) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
/// use malachite_base::num::conversion::mantissa_and_exponent::*;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::rounding_modes::RoundingMode::{self, *};
/// use std::cmp::Ordering::{self, *};
///
/// fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
///     n: T,
///     rm: RoundingMode,
///     out: Option<(U, u64, Ordering)>,
/// ) {
///     assert_eq!(
///         sci_mantissa_and_exponent_round(n, rm).map(|(m, e, o)| (NiceFloat(m), e, o)),
///         out.map(|(m, e, o)| (NiceFloat(m), e, o))
///     );
/// }
/// test::<u32, f32>(3, Down, Some((1.5, 1, Equal)));
/// test::<u32, f32>(3, Ceiling, Some((1.5, 1, Equal)));
/// test::<u32, f32>(3, Up, Some((1.5, 1, Equal)));
/// test::<u32, f32>(3, Nearest, Some((1.5, 1, Equal)));
/// test::<u32, f32>(3, Exact, Some((1.5, 1, Equal)));
///
/// test::<u32, f32>(123, Floor, Some((1.921875, 6, Equal)));
/// test::<u32, f32>(123, Down, Some((1.921875, 6, Equal)));
/// test::<u32, f32>(123, Ceiling, Some((1.921875, 6, Equal)));
/// test::<u32, f32>(123, Up, Some((1.921875, 6, Equal)));
/// test::<u32, f32>(123, Nearest, Some((1.921875, 6, Equal)));
/// test::<u32, f32>(123, Exact, Some((1.921875, 6, Equal)));
///
/// test::<u32, f32>(1000000000, Nearest, Some((1.8626451, 29, Equal)));
/// test::<u32, f32>(999999999, Nearest, Some((1.8626451, 29, Greater)));
/// ```
pub fn sci_mantissa_and_exponent_round<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    x: T,
    rm: RoundingMode,
) -> Option<(U, u64, Ordering)> {
    assert_ne!(x, T::ZERO);
    let significant_bits = x.significant_bits();
    let mut exponent = significant_bits - 1;
    let (mut raw_mantissa, o) = if significant_bits > U::MANTISSA_WIDTH {
        let shift = significant_bits - U::MANTISSA_WIDTH - 1;
        if rm == Exact && TrailingZeros::trailing_zeros(x) < shift {
            return None;
        }
        let (s, o) = x.shr_round(shift, rm);
        (s.wrapping_into(), o)
    } else {
        let x: u64 = x.wrapping_into();
        (x << (U::MANTISSA_WIDTH - significant_bits + 1), Equal)
    };
    if raw_mantissa.significant_bits() == U::MANTISSA_WIDTH + 2 {
        // Rounding up to a power of 2
        raw_mantissa >>= 1;
        exponent += 1;
    }
    raw_mantissa.clear_bit(U::MANTISSA_WIDTH);
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    Some((
        U::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(U::MAX_EXPONENT)),
        exponent,
        o,
    ))
}

/// Constructs a primitive integer from its scientific mantissa and exponent. An [`Ordering`] is
/// also returned, indicating whether the returned value is less than, equal to, or greater than the
/// exact value implied by the input.
///
/// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is a
/// rational number with $1 \leq m_s < 2$. Here, the rational mantissa is provided as a float. If
/// the mantissa is outside the range $[1, 2)$, `None` is returned.
///
/// Some combinations of mantissas and exponents do not specify an integer, in which case the
/// resulting value is rounded to an integer using the specified rounding mode. If the rounding mode
/// is `Exact` but the input does not exactly specify an integer, `None` is returned.
///
/// $$
/// f(x, r) \approx 2^{e_s}m_s.
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `sci_mantissa` is zero.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
/// use malachite_base::num::conversion::mantissa_and_exponent::*;
/// use malachite_base::rounding_modes::RoundingMode::{self, *};
/// use std::cmp::Ordering::{self, *};
///
/// fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
///     mantissa: U,
///     exponent: u64,
///     rm: RoundingMode,
///     out: Option<(T, Ordering)>,
/// ) {
///     assert_eq!(
///         from_sci_mantissa_and_exponent_round::<T, U>(mantissa, exponent, rm),
///         out
///     );
/// }
/// test::<u32, f32>(1.5, 1, Floor, Some((3, Equal)));
/// test::<u32, f32>(1.5, 1, Down, Some((3, Equal)));
/// test::<u32, f32>(1.5, 1, Ceiling, Some((3, Equal)));
/// test::<u32, f32>(1.5, 1, Up, Some((3, Equal)));
/// test::<u32, f32>(1.5, 1, Nearest, Some((3, Equal)));
/// test::<u32, f32>(1.5, 1, Exact, Some((3, Equal)));
///
/// test::<u32, f32>(1.51, 1, Floor, Some((3, Less)));
/// test::<u32, f32>(1.51, 1, Down, Some((3, Less)));
/// test::<u32, f32>(1.51, 1, Ceiling, Some((4, Greater)));
/// test::<u32, f32>(1.51, 1, Up, Some((4, Greater)));
/// test::<u32, f32>(1.51, 1, Nearest, Some((3, Less)));
/// test::<u32, f32>(1.51, 1, Exact, None);
///
/// test::<u32, f32>(2.0, 1, Floor, None);
/// test::<u32, f32>(10.0, 1, Floor, None);
/// test::<u32, f32>(0.5, 1, Floor, None);
/// ```
pub fn from_sci_mantissa_and_exponent_round<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    sci_mantissa: U,
    sci_exponent: u64,
    rm: RoundingMode,
) -> Option<(T, Ordering)> {
    assert_ne!(sci_mantissa, U::ZERO);
    if sci_mantissa < U::ONE || sci_mantissa >= U::TWO {
        return None;
    }
    let mut raw_mantissa = sci_mantissa.raw_mantissa();
    raw_mantissa.set_bit(U::MANTISSA_WIDTH);
    if sci_exponent >= U::MANTISSA_WIDTH {
        T::try_from(raw_mantissa)
            .ok()?
            .arithmetic_checked_shl(sci_exponent - U::MANTISSA_WIDTH)
            .map(|n| (n, Equal))
    } else {
        let shift = U::MANTISSA_WIDTH - sci_exponent;
        if rm == Exact && TrailingZeros::trailing_zeros(raw_mantissa) < shift {
            return None;
        }
        let (s, o) = raw_mantissa.shr_round(shift, rm);
        T::try_from(s).ok().map(|s| (s, o))
    }
}

macro_rules! impl_mantissa_and_exponent_unsigned {
    ($t:ident) => {
        impl IntegerMantissaAndExponent<$t, u64> for $t {
            /// Returns the integer mantissa and exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// The inverse operation is
            /// [`from_integer_mantissa_and_exponent`](Self::from_integer_mantissa_and_exponent).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_mantissa_and_exponent).
            #[inline]
            fn integer_mantissa_and_exponent(self) -> ($t, u64) {
                assert_ne!(self, 0);
                let exponent = TrailingZeros::trailing_zeros(self);
                (self >> exponent, exponent)
            }

            /// Returns the integer mantissa.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = \frac{|x|}{2^{e_i}},
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_mantissa).
            #[inline]
            fn integer_mantissa(self) -> $t {
                assert_ne!(self, 0);
                self >> self.trailing_zeros()
            }

            /// Returns the integer exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = e_i,
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_exponent).
            #[inline]
            fn integer_exponent(self) -> u64 {
                assert_ne!(self, 0);
                TrailingZeros::trailing_zeros(self)
            }

            /// Constructs an unsigned primitive integer from its integer mantissa and exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i,
            /// $$
            /// or `None` if the result cannot be exactly represented as an integer of the desired
            /// type (this happens if the exponent is too large).
            ///
            /// The input does not have to be reduced; that is to say, the mantissa does not have to
            /// be odd.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_integer_mantissa_and_exponent).
            #[inline]
            fn from_integer_mantissa_and_exponent(
                integer_mantissa: $t,
                integer_exponent: u64,
            ) -> Option<$t> {
                integer_mantissa.arithmetic_checked_shl(integer_exponent)
            }
        }
    };
}
apply_to_unsigneds!(impl_mantissa_and_exponent_unsigned);

macro_rules! impl_sci_mantissa_and_exponent_unsigned {
    ($u:ident) => {
        macro_rules! impl_sci_mantissa_and_exponent_unsigned_inner {
            ($f:ident) => {
                impl SciMantissaAndExponent<$f, u64> for $u {
                    /// Returns the scientific mantissa and exponent.
                    ///
                    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an
                    /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent
                    /// the rational mantissa as a float. The conversion might not be exact, so we
                    /// round to the nearest float using the `Nearest` rounding mode. To use other
                    /// rounding modes, use [`sci_mantissa_and_exponent_round`].
                    ///
                    /// If the result cannot be expressed as an integer of the specified type,
                    /// `None` is returned.
                    /// $$
                    /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}},
                    /// \lfloor \log_2 x \rfloor).
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `self` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
                    #[inline]
                    fn sci_mantissa_and_exponent(self) -> ($f, u64) {
                        let (m, e, _) = sci_mantissa_and_exponent_round(self, Nearest).unwrap();
                        (m, e)
                    }

                    /// Constructs a primitive integer from its scientific mantissa and exponent.
                    ///
                    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an
                    /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the
                    /// rational mantissa is provided as a float. If the mantissa is outside the
                    /// range $[1, 2)$, `None` is returned.
                    ///
                    /// Some combinations of mantissas and exponents do not specify an integer, in
                    /// which case the resulting value is rounded to an integer using the `Nearest`
                    /// rounding mode. To specify other rounding modes, use
                    /// [`from_sci_mantissa_and_exponent_round`].
                    ///
                    /// $$
                    /// f(x) \approx 2^{e_s}m_s.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `sci_mantissa` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
                    #[inline]
                    fn from_sci_mantissa_and_exponent(
                        sci_mantissa: $f,
                        sci_exponent: u64,
                    ) -> Option<$u> {
                        from_sci_mantissa_and_exponent_round(sci_mantissa, sci_exponent, Nearest)
                            .map(|p| p.0)
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_sci_mantissa_and_exponent_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_sci_mantissa_and_exponent_unsigned);

macro_rules! impl_mantissa_and_exponent_primitive_float {
    ($t:ident) => {
        impl RawMantissaAndExponent<u64, u64> for $t {
            /// Returns the raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of `self`. When `self` is nonzero and finite, the raw exponent $e_r$ is
            /// an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite, $f(x) = (m_r, e_r)$, where
            /// $$
            /// m_r = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & \text{if} \\quad |x| < 2^{2-2^{E-1},} \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right ) &
            ///     \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// and
            /// $$
            /// e_r = \\begin{cases}
            ///     0 & \text{if} \\quad |x| < 2^{2-2^{E-1}} \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            /// and $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to [IEEE
            /// 754](https://standards.ieee.org/ieee/754/6210/) or look at the examples below.
            ///
            /// The inverse operation is [`Self::from_raw_mantissa_and_exponent`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#raw_mantissa_and_exponent).
            #[inline]
            fn raw_mantissa_and_exponent(self) -> (u64, u64) {
                raw_mantissa_and_exponent(self)
            }

            /// Returns the raw mantissa.
            ///
            /// The raw mantissa is the actual bit pattern used to represent the mantissa of `self`.
            /// When `self` is nonzero and finite, it is an integer in $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & \text{if} \\quad |x| < 2^{2-2^{E-1}}, \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right )
            ///     & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to [IEEE
            /// 754](https://standards.ieee.org/ieee/754/6210/) or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#raw_mantissa).
            #[inline]
            fn raw_mantissa(self) -> u64 {
                raw_mantissa(self)
            }

            /// Returns the raw exponent.
            ///
            /// The raw exponent is the actual bit pattern used to represent the exponent of `self`.
            /// When `self` is nonzero and finite, it is an integer in $[0, 2^E-2]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     0 & \text{if} \\quad |x| < 2^{2-2^{E-1}}, \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to [IEEE
            /// 754](https://standards.ieee.org/ieee/754/6210/) or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#raw_exponent).
            #[inline]
            fn raw_exponent(self) -> u64 {
                raw_exponent(self)
            }

            /// Constructs a float from its raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of a float. When the float is nonzero and finite, the raw exponent $e_r$
            /// is an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in $[0,
            /// 2^M-1]$.
            ///
            /// When the exponent is not `2^E-1`,
            /// $$
            /// f(m_r, e_r) = \\begin{cases}
            ///     2^{2-2^{E-1}-M}m_r & \text{if} \\quad e_r = 0, \\\\
            ///     2^{e_r-2^{E-1}+1}(2^{-M}m_r+1) & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to [IEEE
            /// 754](https://standards.ieee.org/ieee/754/6210/) or look at the examples below.
            ///
            /// This function only outputs a single, canonical, `NaN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_raw_mantissa_and_exponent).
            #[inline]
            fn from_raw_mantissa_and_exponent(raw_mantissa: u64, raw_exponent: u64) -> $t {
                from_raw_mantissa_and_exponent(raw_mantissa, raw_exponent)
            }
        }

        impl IntegerMantissaAndExponent<u64, i64> for $t {
            /// Returns the integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// The inverse operation is [`Self::from_integer_mantissa_and_exponent`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_mantissa_and_exponent).
            #[inline]
            fn integer_mantissa_and_exponent(self) -> (u64, i64) {
                integer_mantissa_and_exponent_primitive_float(self)
            }

            /// Returns the integer mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = \frac{|x|}{2^{e_i}},
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_mantissa).
            #[inline]
            fn integer_mantissa(self) -> u64 {
                integer_mantissa_primitive_float(self)
            }

            /// Returns the integer exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = e_i,
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#integer_exponent).
            #[inline]
            fn integer_exponent(self) -> i64 {
                integer_exponent_primitive_float(self)
            }

            /// Constructs a float from its integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired type
            /// (this happens if the exponent is too large or too small, or if the mantissa's
            /// precision is too high for the exponent).
            ///
            /// The input does not have to be reduced; that is to say, the mantissa does not have to
            /// be odd.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_integer_mantissa_and_exponent).
            #[inline]
            fn from_integer_mantissa_and_exponent(
                integer_mantissa: u64,
                integer_exponent: i64,
            ) -> Option<$t> {
                from_integer_mantissa_and_exponent_primitive_float(
                    integer_mantissa,
                    integer_exponent,
                )
            }
        }

        impl SciMantissaAndExponent<$t, i64> for $t {
            /// Returns the scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$ is
            /// a valid float, the scientific mantissa $m_s$ is always exactly representable as a
            /// float of the same type. We have
            /// $$
            /// f(x) = (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// The inverse operation is `from_sci_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa_and_exponent).
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                sci_mantissa_and_exponent_primitive_float(self)
            }

            /// Returns the scientific mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$ is
            /// a valid float, the scientific mantissa $m_s$ is always exactly representable as a
            /// float of the same type. We have
            /// $$
            /// f(x) = \frac{x}{2^{\lfloor \log_2 x \rfloor}}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_mantissa).
            #[inline]
            fn sci_mantissa(self) -> $t {
                sci_mantissa_primitive_float(self)
            }

            /// Returns the scientific exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We have
            /// $$
            /// f(x) = \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#sci_exponent).
            #[inline]
            fn sci_exponent(self) -> i64 {
                sci_exponent_primitive_float(self)
            }

            /// Constructs a float from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$.
            ///
            /// $$
            /// f(x) = 2^{e_s}m_s,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired type
            /// (this happens if the exponent is too large or too small, if the mantissa is not in
            /// the range $[1, 2)$, or if the mantissa's precision is too high for the exponent).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `mantissa` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See [here](super::mantissa_and_exponent#from_sci_mantissa_and_exponent).
            #[inline]
            fn from_sci_mantissa_and_exponent(sci_mantissa: $t, sci_exponent: i64) -> Option<$t> {
                from_sci_mantissa_and_exponent_primitive_float(sci_mantissa, sci_exponent)
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent_primitive_float);
