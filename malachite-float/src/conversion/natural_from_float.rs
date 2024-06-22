// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, Zero};
use crate::{significand_bits, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, ShrRound};
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;

impl RoundingFrom<Float> for Natural {
    /// Converts a [`Float`] to a [`Natural`], using a specified [`RoundingMode`] and taking the
    /// [`Float`] by value. An [`Ordering`] is also returned, indicating whether the returned value
    /// is less than, equal to, or greater than the original value.
    ///
    /// If the [`Float`] is negative (including negative infinity), then it will be rounded to zero
    /// when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will
    /// panic.
    ///
    /// If the [`Float`] is NaN or positive infinity, the function will panic regardless of the
    /// rounding mode.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`] is less
    /// than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`, or if the [`Float`] is NaN or
    /// positive infinity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, Down).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, Ceiling).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, Nearest).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(f: Float, rm: RoundingMode) -> (Natural, Ordering) {
        match f {
            float_either_zero!() => (Natural::ZERO, Equal),
            float_negative_infinity!() => match rm {
                Ceiling | Down | Nearest => (Natural::ZERO, Greater),
                _ => panic!("Can't convert -Infinity to Natural using {rm}"),
            },
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if !sign {
                    match rm {
                        Ceiling | Down | Nearest => (Natural::ZERO, Greater),
                        _ => panic!("Cannot convert negative number to Natural using {rm}"),
                    }
                } else if exponent < 0 {
                    match rm {
                        Floor | Down | Nearest => (Natural::ZERO, Less),
                        Ceiling | Up => (Natural::ONE, Greater),
                        Exact => panic!("Cannot convert Float to Natural using {rm}"),
                    }
                } else {
                    let sb = significand_bits(&significand);
                    let eb = u64::from(exponent.unsigned_abs());
                    if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Equal)
                    }
                }
            }
            _ => panic!("Can't convert {f} to Natural using {rm}"),
        }
    }
}

impl<'a> RoundingFrom<&'a Float> for Natural {
    /// Converts a [`Float`] to a [`Natural`], using a specified [`RoundingMode`] and taking the
    /// [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Float`] is negative (including negative infinity), then it will be rounded to zero
    /// when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will
    /// panic.
    ///
    /// If the [`Float`] is NaN or positive infinity, the function will panic regardless of the
    /// rounding mode.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`] is less
    /// than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`, or if the [`Float`] is NaN or
    /// positive infinity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, Down).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, Ceiling).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, Nearest).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(f: &'a Float, rm: RoundingMode) -> (Natural, Ordering) {
        match f {
            float_either_zero!() => (Natural::ZERO, Equal),
            float_negative_infinity!() => match rm {
                Ceiling | Down | Nearest => (Natural::ZERO, Greater),
                _ => panic!("Can't convert -Infinity to Natural using {rm}"),
            },
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if !sign {
                    match rm {
                        Ceiling | Down | Nearest => (Natural::ZERO, Greater),
                        _ => panic!("Cannot convert -Infinity to Natural using {rm}"),
                    }
                } else if *exponent < 0 {
                    match rm {
                        Floor | Down | Nearest => (Natural::ZERO, Less),
                        Ceiling | Up => (Natural::ONE, Greater),
                        Exact => panic!("Cannot convert Float to Natural using {rm}"),
                    }
                } else {
                    let sb = significand_bits(significand);
                    let eb = u64::from(exponent.unsigned_abs());
                    if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Equal)
                    }
                }
            }
            _ => panic!("Can't convert {f} to Natural using {rm}"),
        }
    }
}

impl TryFrom<Float> for Natural {
    type Error = UnsignedFromFloatError;

    /// Converts a [`Float`] to a [`Natural`], taking the [`Float`] by value. If the [`Float`] is
    /// not equal to a non-negative integer, an error is returned.
    ///
    /// Both positive and negative zero convert to a [`Natural`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::from::UnsignedFromFloatError::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::try_from(Float::ZERO).unwrap(), 0);
    /// assert_eq!(Natural::try_from(Float::from(123.0)).unwrap(), 123);
    ///
    /// assert_eq!(Natural::try_from(Float::from(-123.0)), Err(FloatNegative));
    /// assert_eq!(
    ///     Natural::try_from(Float::from(1.5)),
    ///     Err(FloatNonIntegerOrOutOfRange)
    /// );
    /// assert_eq!(Natural::try_from(Float::INFINITY), Err(FloatInfiniteOrNan));
    /// assert_eq!(Natural::try_from(Float::NAN), Err(FloatInfiniteOrNan));
    /// ```
    fn try_from(f: Float) -> Result<Natural, Self::Error> {
        match f {
            float_either_zero!() => Ok(Natural::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if !sign {
                    Err(UnsignedFromFloatError::FloatNegative)
                } else if exponent <= 0 {
                    Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                } else {
                    let sb = significand_bits(&significand);
                    let eb = u64::from(exponent.unsigned_abs());
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            Ok(significand >> bits)
                        } else {
                            Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                        }
                    } else {
                        Ok(significand << (eb - sb))
                    }
                }
            }
            _ => Err(UnsignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl<'a> TryFrom<&'a Float> for Natural {
    type Error = UnsignedFromFloatError;

    /// Converts a [`Float`] to a [`Natural`], taking the [`Float`] by reference. If the [`Float`]
    /// is not equal to a non-negative integer, an error is returned.
    ///
    /// Both positive and negative zero convert to a [`Natural`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::from::UnsignedFromFloatError::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::try_from(&Float::ZERO).unwrap(), 0);
    /// assert_eq!(Natural::try_from(&Float::from(123.0)).unwrap(), 123);
    ///
    /// assert_eq!(Natural::try_from(&Float::from(-123.0)), Err(FloatNegative));
    /// assert_eq!(
    ///     Natural::try_from(&Float::from(1.5)),
    ///     Err(FloatNonIntegerOrOutOfRange)
    /// );
    /// assert_eq!(Natural::try_from(&Float::INFINITY), Err(FloatInfiniteOrNan));
    /// assert_eq!(Natural::try_from(&Float::NAN), Err(FloatInfiniteOrNan));
    /// ```
    fn try_from(f: &'a Float) -> Result<Natural, Self::Error> {
        match f {
            float_either_zero!() => Ok(Natural::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if !*sign {
                    Err(UnsignedFromFloatError::FloatNegative)
                } else if *exponent <= 0 {
                    Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                } else {
                    let sb = significand_bits(significand);
                    let eb = u64::from(exponent.unsigned_abs());
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            Ok(significand >> bits)
                        } else {
                            Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                        }
                    } else {
                        Ok(significand << (eb - sb))
                    }
                }
            }
            _ => Err(UnsignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl<'a> ConvertibleFrom<&'a Float> for Natural {
    /// Determines whether a [`Float`] can be converted to a [`Natural`] (when the [`Float`] is
    /// non-negative and an integer), taking the [`Float`] by reference.
    ///
    /// Both positive and negative zero are convertible to a [`Natural`]. (Although negative zero is
    /// nominally negative, the real number it represents is zero, which is not negative.)
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::convertible_from(&Float::ZERO), true);
    /// assert_eq!(Natural::convertible_from(&Float::from(123.0)), true);
    ///
    /// assert_eq!(Natural::convertible_from(&Float::from(-123.0)), false);
    /// assert_eq!(Natural::convertible_from(&Float::from(1.5)), false);
    /// assert_eq!(Natural::convertible_from(&Float::INFINITY), false);
    /// assert_eq!(Natural::convertible_from(&Float::NAN), false);
    /// ```
    fn convertible_from(f: &'a Float) -> bool {
        match f {
            float_either_zero!() => true,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *sign && *exponent > 0 && {
                    let sb = significand_bits(significand);
                    let eb = u64::from(exponent.unsigned_abs());
                    sb < eb || significand.divisible_by_power_of_2(sb - eb)
                }
            }
            _ => false,
        }
    }
}
