// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

const fn const_limb_significant_bits(x: Limb) -> u64 {
    Limb::WIDTH - (x.leading_zeros() as u64)
}

impl Float {
    /// Converts an unsigned primitive integer to a [`Float`], after multiplying it by the specified
    /// power of 2.
    ///
    /// The type of the integer is `u64`, unless the `32_bit_limbs` feature is set, in which case
    /// the type is `u32`.
    ///
    /// If the integer is nonzero, the precision of the [`Float`] is the minimum possible precision
    /// to represent the integer exactly.
    ///
    /// If you don't need to use this function in a const context, try just using `from` instead,
    /// followed by `>>` or `<<`.
    ///
    /// $$
    /// f(x,k) = x2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the result is too large or too small to be represented by a `Float`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::const_from_unsigned_times_power_of_2(0, 0).to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_unsigned_times_power_of_2(123, 0).to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_unsigned_times_power_of_2(123, 1).to_string(),
    ///     "246.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_unsigned_times_power_of_2(123, -1).to_string(),
    ///     "61.5"
    /// );
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// {
    ///     assert_eq!(
    ///         Float::const_from_unsigned_times_power_of_2(884279719003555, -48).to_string(),
    ///         "3.141592653589793"
    ///     );
    /// }
    /// ```
    pub const fn const_from_unsigned_times_power_of_2(x: Limb, pow: i32) -> Float {
        if x == 0 {
            return Float::ZERO;
        }
        let bits = const_limb_significant_bits(x);
        let bits_i32 = bits as i32;
        let exponent = bits_i32.saturating_add(pow);
        assert!(exponent <= Float::MAX_EXPONENT);
        assert!(exponent >= Float::MIN_EXPONENT);
        let prec = bits - x.trailing_zeros() as u64;
        let mut limbs = prec >> Limb::LOG_WIDTH;
        if prec & Limb::WIDTH_MASK != 0 {
            limbs += 1;
        }
        Float(Finite {
            sign: true,
            exponent,
            precision: prec,
            significand: Natural::const_from(x << ((limbs << Limb::LOG_WIDTH) - bits)),
        })
    }

    /// Converts an unsigned primitive integer to a [`Float`].
    ///
    /// The type of the integer is `u64`, unless the `32_bit_limbs` feature is set, in which case
    /// the type is `u32`.
    ///
    /// If the integer is nonzero, the precision of the [`Float`] is the minimum possible precision
    /// to represent the integer exactly.
    ///
    /// If you don't need to use this function in a const context, try just using `from` instead; it
    /// will probably be slightly faster.
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::const_from_unsigned(0).to_string(), "0.0");
    /// assert_eq!(Float::const_from_unsigned(123).to_string(), "123.0");
    /// ```
    #[inline]
    pub const fn const_from_unsigned(x: Limb) -> Float {
        Float::const_from_unsigned_times_power_of_2(x, 0)
    }

    /// Converts a signed primitive integer to a [`Float`], after multiplying it by the specified
    /// power of 2.
    ///
    /// The type of the integer is `i64`, unless the `32_bit_limbs` feature is set, in which case
    /// the type is `i32`.
    ///
    /// If the integer is nonzero, the precision of the [`Float`] is the minimum possible precision
    /// to represent the integer exactly.
    ///
    /// If you don't need to use this function in a const context, try just using `from` instead,
    /// followed by `>>` or `<<`.
    ///
    /// $$
    /// f(x,k) = x2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the result is too large or too small to be represented by a `Float`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(0, 0).to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(123, 0).to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(123, 1).to_string(),
    ///     "246.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(123, -1).to_string(),
    ///     "61.5"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(-123, 0).to_string(),
    ///     "-123.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(-123, 1).to_string(),
    ///     "-246.0"
    /// );
    /// assert_eq!(
    ///     Float::const_from_signed_times_power_of_2(-123, -1).to_string(),
    ///     "-61.5"
    /// );
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// {
    ///     assert_eq!(
    ///         Float::const_from_signed_times_power_of_2(884279719003555, -48).to_string(),
    ///         "3.141592653589793"
    ///     );
    ///     assert_eq!(
    ///         Float::const_from_signed_times_power_of_2(-884279719003555, -48).to_string(),
    ///         "-3.141592653589793"
    ///     );
    /// }
    /// ```
    pub const fn const_from_signed_times_power_of_2(x: SignedLimb, pow: i32) -> Float {
        if x == 0 {
            return Float::ZERO;
        }
        let x_abs = x.unsigned_abs();
        let bits = const_limb_significant_bits(x_abs);
        let bits_i32 = bits as i32;
        let exponent = bits_i32.saturating_add(pow);
        assert!(exponent <= Float::MAX_EXPONENT);
        assert!(exponent >= Float::MIN_EXPONENT);
        let prec = bits - x_abs.trailing_zeros() as u64;
        let mut limbs = prec >> Limb::LOG_WIDTH;
        if prec & Limb::WIDTH_MASK != 0 {
            limbs += 1;
        }
        Float(Finite {
            sign: x > 0,
            exponent,
            precision: prec,
            significand: Natural::const_from(x_abs << ((limbs << Limb::LOG_WIDTH) - bits)),
        })
    }

    /// Converts a signed primitive integer to a [`Float`].
    ///
    /// The type of the integer is `i64`, unless the `32_bit_limbs` feature is set, in which case
    /// the type is `i32`.
    ///
    /// If the integer is nonzero, the precision of the [`Float`] is the minimum possible precision
    /// to represent the integer exactly.
    ///
    /// If you don't need to use this function in a const context, try just using `from` instead; it
    /// will probably be slightly faster.
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::const_from_signed(0).to_string(), "0.0");
    /// assert_eq!(Float::const_from_signed(123).to_string(), "123.0");
    /// assert_eq!(Float::const_from_signed(-123).to_string(), "-123.0");
    /// ```
    #[inline]
    pub const fn const_from_signed(x: SignedLimb) -> Float {
        Float::const_from_signed_times_power_of_2(x, 0)
    }

    /// Converts a primitive unsigned integer to a [`Float`]. If the [`Float`] is nonzero, it has
    /// the specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value.
    ///
    /// If you're only using `Nearest`, try using [`Float::from_unsigned_prec`] instead.
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the primitive integer cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_unsigned_prec_round).
    #[inline]
    pub fn from_unsigned_prec_round<T: PrimitiveUnsigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_prec_round(Natural::from(x), prec, rm)
    }

    /// Converts an unsigned primitive integer to a [`Float`]. If the [`Float`] is nonzero, it has
    /// the specified precision. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the integer's number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case `Nearest` is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_unsigned_prec_round`].
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_unsigned_prec).
    #[inline]
    pub fn from_unsigned_prec<T: PrimitiveUnsigned>(x: T, prec: u64) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_prec(Natural::from(x), prec)
    }

    /// Converts a primitive signed integer to a [`Float`]. If the [`Float`] is nonzero, it has the
    /// specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value.
    ///
    /// If you're only using `Nearest`, try using [`Float::from_signed_prec`] instead.
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the primitive integer cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_signed_prec_round).
    #[inline]
    pub fn from_signed_prec_round<T: PrimitiveSigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_prec_round(Integer::from(x), prec, rm)
    }

    /// Converts a signed primitive integer to a [`Float`]. If the [`Float`] is nonzero, it has the
    /// specified precision. An [`Ordering`] is also returned, indicating whether the returned value
    /// is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the integer's number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case `Nearest` is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_signed_prec_round`].
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_signed_prec).
    #[inline]
    pub fn from_signed_prec<T: PrimitiveSigned>(x: T, prec: u64) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_prec(Integer::from(x), prec)
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Float {
            /// Converts an unsigned primitive integer to a [`Float`].
            ///
            /// If the integer is nonzero, the precision of the [`Float`] is equal to the integer's
            /// number of significant bits. If you want to specify a different precision, try
            /// [`Float::from_unsigned_prec`]. This may require rounding, which uses `Nearest` by
            /// default. To specify a rounding mode as well as a precision, try
            /// [`Float::from_unsigned_prec_round`].
            ///
            /// If you want to create a [`Float`] from an unsigned primitive integer in a const
            /// context, try [`Float::const_from_unsigned`] instead.
            ///
            /// This function does not overflow or underflow.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Float {
                Float::exact_from(Natural::from(u))
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Float {
            /// Converts a signed primitive integer to a [`Float`].
            ///
            /// If the integer is nonzero, the precision of the [`Float`] is equal to the integer's
            /// number of significant bits. If you want to specify a different precision, try
            /// [`Float::from_signed_prec`]. This may require rounding, which uses `Nearest` by
            /// default. To specify a rounding mode as well as a precision, try
            /// [`Float::from_signed_prec_round`].
            ///
            /// If you want to create a [`Float`] from an signed primitive integer in a const
            /// context, try [`Float::const_from_signed`] instead.
            ///
            /// This function does not overflow or underflow.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Float {
                Float::exact_from(Integer::from(i))
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
