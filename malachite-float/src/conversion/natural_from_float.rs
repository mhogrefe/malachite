use crate::InnerFloat::{Finite, Infinity, Zero};
use crate::{significand_bits, Float};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, ShrRound};
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
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
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::from(1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, RoundingMode::Down).to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Float::NEGATIVE_INFINITY, RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(f: Float, rm: RoundingMode) -> (Natural, Ordering) {
        match f {
            float_either_zero!() => (Natural::ZERO, Ordering::Equal),
            float_negative_infinity!() => match rm {
                RoundingMode::Ceiling | RoundingMode::Down | RoundingMode::Nearest => {
                    (Natural::ZERO, Ordering::Greater)
                }
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
                        RoundingMode::Ceiling | RoundingMode::Down | RoundingMode::Nearest => {
                            (Natural::ZERO, Ordering::Greater)
                        }
                        _ => panic!("Cannot convert negative number to Natural using {rm}"),
                    }
                } else if exponent < 0 {
                    match rm {
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            (Natural::ZERO, Ordering::Less)
                        }
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (Natural::ONE, Ordering::Greater)
                        }
                        RoundingMode::Exact => panic!("Cannot convert Float to Natural using {rm}"),
                    }
                } else {
                    let sb = significand_bits(&significand);
                    let eb = exponent.unsigned_abs();
                    if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Ordering::Equal)
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
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::from(1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Float::NEGATIVE_INFINITY, RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    fn rounding_from(f: &'a Float, rm: RoundingMode) -> (Natural, Ordering) {
        match f {
            float_either_zero!() => (Natural::ZERO, Ordering::Equal),
            float_negative_infinity!() => match rm {
                RoundingMode::Ceiling | RoundingMode::Down | RoundingMode::Nearest => {
                    (Natural::ZERO, Ordering::Greater)
                }
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
                        RoundingMode::Ceiling | RoundingMode::Down | RoundingMode::Nearest => {
                            (Natural::ZERO, Ordering::Greater)
                        }
                        _ => panic!("Cannot convert -Infinity to Natural using {rm}"),
                    }
                } else if *exponent < 0 {
                    match rm {
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            (Natural::ZERO, Ordering::Less)
                        }
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (Natural::ONE, Ordering::Greater)
                        }
                        RoundingMode::Exact => panic!("Cannot convert Float to Natural using {rm}"),
                    }
                } else {
                    let sb = significand_bits(significand);
                    let eb = exponent.unsigned_abs();
                    if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Ordering::Equal)
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
    /// assert_eq!(Natural::try_from(Float::from(1.5)), Err(FloatNonIntegerOrOutOfRange));
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
                    let eb = exponent.unsigned_abs();
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
    /// assert_eq!(Natural::try_from(&Float::from(1.5)), Err(FloatNonIntegerOrOutOfRange));
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
                    let eb = exponent.unsigned_abs();
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
                    let eb = exponent.unsigned_abs();
                    sb < eb || significand.divisible_by_power_of_2(sb - eb)
                }
            }
            _ => false,
        }
    }
}
