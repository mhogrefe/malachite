// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

const fn const_limb_significant_bits(x: Limb) -> u64 {
    Limb::WIDTH - (x.leading_zeros() as u64)
}

const fn const_u64_power_of_2(pow: u64) -> u64 {
    assert!(pow < Limb::WIDTH);
    1 << pow
}

const fn const_u64_low_mask(bits: u64) -> u64 {
    assert!(bits <= Limb::WIDTH);
    if bits == Limb::WIDTH {
        u64::MAX
    } else {
        const_u64_power_of_2(bits) - 1
    }
}

const fn const_u64_mod_power_of_2(x: u64, pow: u64) -> u64 {
    if x == 0 || pow >= Limb::WIDTH {
        x
    } else {
        x & const_u64_low_mask(pow)
    }
}

const fn const_u64_neg_mod_power_of_2(x: u64, pow: u64) -> u64 {
    assert!(x == 0 || pow <= Limb::WIDTH);
    const_u64_mod_power_of_2(x.wrapping_neg(), pow)
}

const fn const_i64_convertible_from_limb(value: Limb) -> bool {
    (value as i64 as Limb) == value
}

impl Float {
    // TODO test
    pub const fn const_from_unsigned(x: Limb) -> Float {
        if x == 0 {
            return Float::ZERO;
        }
        assert!(const_i64_convertible_from_limb(x));
        let bits = const_limb_significant_bits(x);
        Float(Finite {
            sign: true,
            exponent: bits as i32,
            precision: bits,
            significand: Natural::const_from(
                // TODO simplify?
                x << const_u64_neg_mod_power_of_2(bits, Limb::LOG_WIDTH),
            ),
        })
    }

    /// Converts a primitive unsigned integer to a [`Float`]. If the [`Float`] is nonzero, it has
    /// the specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value.
    ///
    /// If you're only using `Nearest`, try using [`Float::from_unsigned_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Float {
                Float::from(Natural::from(u))
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
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Float {
                Float::from(Integer::from(i))
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
