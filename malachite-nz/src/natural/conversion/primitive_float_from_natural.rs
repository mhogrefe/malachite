// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, RawMantissaAndExponent, RoundingFrom, SciMantissaAndExponent,
    WrappingFrom,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveFloatFromNaturalError;

macro_rules! float_impls {
    ($f: ident) => {
        impl<'a> RoundingFrom<&'a Natural> for $f {
            /// Converts a [`Natural`] to a primitive float according to a specified
            /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the returned
            /// value is less than, equal to, or greater than the original value.
            ///
            /// - If the rounding mode is `Floor` or `Down`, the largest float less than or equal to
            ///   the [`Natural`] is returned. If the [`Natural`] is greater than the maximum finite
            ///   float, then the maximum finite float is returned.
            /// - If the rounding mode is `Ceiling` or `Up`, the smallest float greater than or
            ///   equal to the [`Natural`] is returned. If the [`Natural`] is greater than the
            ///   maximum finite float, then positive infinity is returned.
            /// - If the rounding mode is `Nearest`, then the nearest float is returned. If the
            ///   [`Natural`] is exactly between two floats, the float with the zero
            ///   least-significant bit in its representation is selected. If the [`Natural`] is
            ///   greater than the maximum finite float, then the maximum finite float is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_natural#rounding_from).
            fn rounding_from(value: &'a Natural, rm: RoundingMode) -> ($f, Ordering) {
                if *value == 0 {
                    (0.0, Equal)
                } else {
                    let (mantissa, exponent, o) = value
                        .sci_mantissa_and_exponent_round(rm)
                        .expect("Value cannot be represented exactly as a float");
                    if let Some(f) =
                        $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
                    {
                        (f, o)
                    } else {
                        match rm {
                            Exact => {
                                panic!("Value cannot be represented exactly as an {}", $f::NAME)
                            }
                            Floor | Down | Nearest => ($f::MAX_FINITE, Less),
                            _ => ($f::INFINITY, Greater),
                        }
                    }
                }
            }
        }

        impl<'a> TryFrom<&'a Natural> for $f {
            type Error = PrimitiveFloatFromNaturalError;

            /// Converts a [`Natural`] to a primitive float.
            ///
            /// If the input isn't exactly equal to some float, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_natural#try_from).
            fn try_from(value: &'a Natural) -> Result<$f, Self::Error> {
                if *value == 0 {
                    Ok(0.0)
                } else {
                    let (mantissa, exponent, _) = value
                        .sci_mantissa_and_exponent_round(Exact)
                        .ok_or(PrimitiveFloatFromNaturalError)?;
                    $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
                        .ok_or(PrimitiveFloatFromNaturalError)
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Natural> for $f {
            /// Determines whether a [`Natural`] can be exactly converted to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_natural#convertible_from).
            fn convertible_from(value: &'a Natural) -> bool {
                if *value == 0 {
                    true
                } else {
                    if let Some((mantissa, exponent, _)) =
                        value.sci_mantissa_and_exponent_round::<$f>(Exact)
                    {
                        let exponent = i64::exact_from(exponent);
                        if !($f::MIN_EXPONENT..=$f::MAX_EXPONENT).contains(&exponent) {
                            return false;
                        }
                        let (orig_mantissa, orig_exponent) = mantissa.raw_mantissa_and_exponent();
                        orig_exponent == u64::wrapping_from($f::MAX_EXPONENT)
                            && exponent >= $f::MIN_NORMAL_EXPONENT
                            || orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(
                                $f::MIN_NORMAL_EXPONENT - exponent,
                            ))
                    } else {
                        false
                    }
                }
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
