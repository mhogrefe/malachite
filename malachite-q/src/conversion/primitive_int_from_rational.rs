// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use core::ops::Neg;
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{DivRound, DivisibleByPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedFromRationalError;

fn try_from_unsigned<'a, T: TryFrom<&'a Natural>>(
    x: &'a Rational,
) -> Result<T, UnsignedFromRationalError> {
    if x.sign && x.denominator == 1u32 {
        T::try_from(&x.numerator).map_err(|_| UnsignedFromRationalError)
    } else {
        Err(UnsignedFromRationalError)
    }
}

fn convertible_from_unsigned<T: for<'a> ConvertibleFrom<&'a Natural>>(x: &Rational) -> bool {
    x.sign && x.denominator == 1u32 && T::convertible_from(&x.numerator)
}

#[allow(clippy::let_and_return)] // n doesn't live long enough for a direct return
fn rounding_from_unsigned<'a, T: for<'b> TryFrom<&'b Natural> + Max + Named + Zero>(
    x: &'a Rational,
    rm: RoundingMode,
) -> (T, Ordering) {
    if x.sign {
        let (n, o) = (&x.numerator).div_round(&x.denominator, rm);
        let out = if let Ok(q) = T::try_from(&n) {
            (q, o)
        } else if rm == Down || rm == Floor || rm == Nearest {
            (T::MAX, Less)
        } else {
            panic!(
                "Rational is too large to round to {} using RoundingMode {}",
                rm,
                T::NAME
            );
        };
        out
    } else if rm == Down || rm == Ceiling || rm == Nearest {
        (T::ZERO, Greater)
    } else {
        panic!(
            "Cannot round negative Rational to {} using RoundingMode {}",
            rm,
            T::NAME
        );
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedFromRationalError;

#[allow(clippy::trait_duplication_in_bounds)]
fn try_from_signed<
    'a,
    U: WrappingFrom<&'a Natural>,
    S: Neg<Output = S> + PrimitiveInt + WrappingFrom<U> + WrappingFrom<&'a Natural>,
>(
    x: &'a Rational,
) -> Result<S, SignedFromRationalError> {
    if x.denominator != 1u32 {
        return Err(SignedFromRationalError);
    }
    match *x {
        Rational {
            sign: true,
            ref numerator,
            ..
        } => {
            if numerator.significant_bits() < S::WIDTH {
                Ok(S::wrapping_from(numerator))
            } else {
                Err(SignedFromRationalError)
            }
        }
        Rational {
            sign: false,
            ref numerator,
            ..
        } => {
            let significant_bits = numerator.significant_bits();
            if significant_bits < S::WIDTH
                || significant_bits == S::WIDTH && numerator.divisible_by_power_of_2(S::WIDTH - 1)
            {
                Ok(S::wrapping_from(U::wrapping_from(numerator)).wrapping_neg())
            } else {
                Err(SignedFromRationalError)
            }
        }
    }
}

fn convertible_from_signed<T: PrimitiveInt>(x: &Rational) -> bool {
    if x.denominator != 1u32 {
        return false;
    }
    match *x {
        Rational {
            sign: true,
            ref numerator,
            ..
        } => numerator.significant_bits() < T::WIDTH,
        Rational {
            sign: false,
            ref numerator,
            ..
        } => {
            let significant_bits = numerator.significant_bits();
            significant_bits < T::WIDTH
                || significant_bits == T::WIDTH && numerator.divisible_by_power_of_2(T::WIDTH - 1)
        }
    }
}

fn rounding_from_signed<'a, T: Max + Min + Named + for<'b> WrappingFrom<&'b Integer>>(
    x: &'a Rational,
    rm: RoundingMode,
) -> (T, Ordering)
where
    Integer: PartialOrd<T>,
{
    let (i, o) = Integer::rounding_from(x, rm);
    if i > T::MAX {
        if rm == Down || rm == Floor || rm == Nearest {
            (T::MAX, Less)
        } else {
            panic!(
                "Rational is too large to round to {} using RoundingMode {}",
                rm,
                T::NAME
            );
        }
    } else if i < T::MIN {
        if rm == Down || rm == Ceiling || rm == Nearest {
            (T::MIN, Greater)
        } else {
            panic!(
                "Rational is too small to round to {} using RoundingMode {}",
                rm,
                T::NAME
            );
        }
    } else {
        (T::wrapping_from(&i), o)
    }
}

macro_rules! impl_from_unsigned {
    ($u: ident) => {
        impl<'a> TryFrom<&'a Rational> for $u {
            type Error = UnsignedFromRationalError;

            /// Converts a [`Rational`] to an unsigned primitive integer, returning an error if the
            /// [`Rational`] cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#try_from).
            #[inline]
            fn try_from(value: &Rational) -> Result<$u, UnsignedFromRationalError> {
                try_from_unsigned(value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Rational> for $u {
            /// Determines whether a [`Rational`] can be converted to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#convertible_from).
            #[inline]
            fn convertible_from(value: &Rational) -> bool {
                convertible_from_unsigned::<$u>(value)
            }
        }

        impl<'a> RoundingFrom<&'a Rational> for $u {
            /// Converts a [`Rational`] to an unsigned integer, using a specified [`RoundingMode`].
            ///
            /// If the [`Rational`] is negative, then it will be rounded to zero when `rm` is
            /// `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
            ///
            /// If the [`Rational`] is larger than the maximum value of the unsigned type, then it
            /// will be rounded to the maximum value when `rm` is `Floor`, `Down`, or `Nearest`.
            /// Otherwise, this function will panic.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, if the
            /// [`Rational`] is less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`, or
            /// if the [`Rational`] is greater than `T::MAX` and `rm` is not `Down`, `Floor`, or
            /// `Nearest`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#rounding_from).
            #[inline]
            fn rounding_from(value: &Rational, rm: RoundingMode) -> ($u, Ordering) {
                rounding_from_unsigned(value, rm)
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($u: ident, $s: ident) => {
        impl<'a> TryFrom<&'a Rational> for $s {
            type Error = SignedFromRationalError;

            /// Converts a [`Rational`] to a signed primitive integer, returning an error if the
            /// [`Rational`] cannot be represented.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#try_from).
            #[inline]
            fn try_from(value: &Rational) -> Result<$s, SignedFromRationalError> {
                try_from_signed::<$u, $s>(value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Rational> for $s {
            /// Determines whether a [`Rational`] can be converted to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#convertible_from).
            #[inline]
            fn convertible_from(value: &Rational) -> bool {
                convertible_from_signed::<$s>(value)
            }
        }

        impl<'a> RoundingFrom<&'a Rational> for $s {
            /// Converts a [`Rational`] to a signed integer, using a specified [`RoundingMode`].
            ///
            /// If the [`Rational`] is smaller than the minimum value of the unsigned type, then it
            /// will be rounded to the minimum value when `rm` is `Ceiling`, `Down`, or `Nearest`.
            /// Otherwise, this function will panic.
            ///
            /// If the [`Rational`] is larger than the maximum value of the unsigned type, then it
            /// will be rounded to the maximum value when `rm` is `Floor`, `Down`, or `Nearest`.
            /// Otherwise, this function will panic.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, if the
            /// [`Rational`] is less than `T::MIN` and `rm` is not `Down`, `Ceiling`, or `Nearest`,
            /// or if the [`Rational`] is greater than `T::MAX` and `rm` is not `Down`, `Floor`, or
            /// `Nearest`.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_rational#rounding_from).
            #[inline]
            fn rounding_from(value: &Rational, rm: RoundingMode) -> ($s, Ordering) {
                rounding_from_signed(value, rm)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_from_signed);
