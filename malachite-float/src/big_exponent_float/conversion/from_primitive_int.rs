// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use core::cmp::Ordering;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

impl BigExponentFloat {
    /// Converts an unsigned primitive integer times a power of 2 to a [`BigExponentFloat`],
    /// exactly. This function can be used in `const` contexts.
    ///
    /// The result is $x2^{p}$, where $x$ is `x` and $p$ is `pow`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics (or fails to compile, in a `const` context) if $2^{p}$ times the highest power of 2
    /// at most $x$ exceeds the range that a [`SignedLimb`] exponent can describe, that is, if `pow
    /// + x.significant_bits()` overflows.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// const THREE_EIGHTHS: BigExponentFloat =
    ///     BigExponentFloat::const_from_unsigned_times_power_of_2(3, -3);
    /// assert_eq!(format!("{THREE_EIGHTHS:?}"), "0x0.6");
    /// ```
    pub const fn const_from_unsigned_times_power_of_2(x: Limb, pow: SignedLimb) -> Self {
        if x == 0 {
            return Self::ZERO;
        }
        let zeros = x.leading_zeros();
        Self(Finite {
            sign: true,
            exponent: Integer::const_from_signed(
                pow + (Limb::WIDTH as SignedLimb - zeros as SignedLimb),
            ),
            significand: Natural::const_from(x << zeros),
        })
    }

    /// Converts an unsigned primitive integer to a [`BigExponentFloat`], exactly. This function can
    /// be used in `const` contexts.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// const TEN: BigExponentFloat = BigExponentFloat::const_from_unsigned(10);
    /// assert_eq!(format!("{TEN:?}"), "0xa.0");
    /// ```
    #[inline]
    pub const fn const_from_unsigned(x: Limb) -> Self {
        Self::const_from_unsigned_times_power_of_2(x, 0)
    }

    /// Converts a signed primitive integer times a power of 2 to a [`BigExponentFloat`], exactly.
    /// This function can be used in `const` contexts.
    ///
    /// The result is $x2^{p}$, where $x$ is `x` and $p$ is `pow`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics (or fails to compile, in a `const` context) if `pow + x.unsigned_abs()
    /// .significant_bits()` overflows a [`SignedLimb`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// const NEGATIVE_THREE_EIGHTHS: BigExponentFloat =
    ///     BigExponentFloat::const_from_signed_times_power_of_2(-3, -3);
    /// assert_eq!(format!("{NEGATIVE_THREE_EIGHTHS:?}"), "-0x0.6");
    /// ```
    pub const fn const_from_signed_times_power_of_2(x: SignedLimb, pow: SignedLimb) -> Self {
        let mut y = Self::const_from_unsigned_times_power_of_2(x.unsigned_abs(), pow);
        if x < 0
            && let Self(Finite { sign, .. }) = &mut y
        {
            *sign = false;
        }
        y
    }

    /// Converts a signed primitive integer to a [`BigExponentFloat`], exactly. This function can be
    /// used in `const` contexts.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// const NEGATIVE_TEN: BigExponentFloat = BigExponentFloat::const_from_signed(-10);
    /// assert_eq!(format!("{NEGATIVE_TEN:?}"), "-0xa.0");
    /// ```
    #[inline]
    pub const fn const_from_signed(x: SignedLimb) -> Self {
        Self::const_from_signed_times_power_of_2(x, 0)
    }

    /// Converts an unsigned primitive integer to a [`BigExponentFloat`], rounding to a given
    /// precision with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `x` cannot be represented exactly with
    /// the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_unsigned_prec_round(123u8, 4, Floor);
    /// assert_eq!(format!("{x:?}"), "0x78.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_unsigned_prec_round<T: PrimitiveUnsigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering)
    where
        Natural: From<T>,
    {
        Self::from_natural_prec_round(Natural::from(x), prec, rm)
    }

    /// Converts an unsigned primitive integer to a [`BigExponentFloat`], rounding to the nearest
    /// value of a given precision. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_unsigned_prec(123u8, 4);
    /// assert_eq!(format!("{x:?}"), "0x78.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_unsigned_prec<T: PrimitiveUnsigned>(x: T, prec: u64) -> (Self, Ordering)
    where
        Natural: From<T>,
    {
        Self::from_unsigned_prec_round(x, prec, Nearest)
    }

    /// Converts a signed primitive integer to a [`BigExponentFloat`], rounding to a given precision
    /// with the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `x` cannot be represented exactly with
    /// the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_signed_prec_round(-123i8, 4, Floor);
    /// assert_eq!(format!("{x:?}"), "-0x8.0E+1");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_signed_prec_round<T: PrimitiveSigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering)
    where
        Integer: From<T>,
    {
        Self::from_integer_prec_round(Integer::from(x), prec, rm)
    }

    /// Converts a signed primitive integer to a [`BigExponentFloat`], rounding to the nearest value
    /// of a given precision. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_signed_prec(-123i8, 4);
    /// assert_eq!(format!("{x:?}"), "-0x78.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_signed_prec<T: PrimitiveSigned>(x: T, prec: u64) -> (Self, Ordering)
    where
        Integer: From<T>,
    {
        Self::from_signed_prec_round(x, prec, Nearest)
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for BigExponentFloat {
            /// Converts an unsigned primitive integer to a [`BigExponentFloat`], exactly.
            ///
            /// The result's precision is the number of bits from the most significant 1 of `x` to
            /// its least significant 1.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(x: $t) -> BigExponentFloat {
                BigExponentFloat::from(Natural::from(x))
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for BigExponentFloat {
            /// Converts a signed primitive integer to a [`BigExponentFloat`], exactly.
            ///
            /// The result's precision is the number of bits from the most significant 1 of `x`'s
            /// absolute value to its least significant 1.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(x: $t) -> BigExponentFloat {
                BigExponentFloat::from(Integer::from(x))
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
