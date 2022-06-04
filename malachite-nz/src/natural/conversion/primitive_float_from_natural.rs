use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RawMantissaAndExponent, RoundingFrom,
    SciMantissaAndExponent, WrappingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        impl<'a> RoundingFrom<&'a Natural> for $f {
            /// Converts a [`Natural`] to a primitive float according to a specified
            /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
            ///
            /// - If the rounding mode is `Floor` or `Down`, the largest float less than or equal
            ///   to the [`Natural`] is returned. If the [`Natural`] is greater than the maximum
            ///   finite float, then the maximum finite float is returned.
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
            fn rounding_from(value: &'a Natural, rm: RoundingMode) -> $f {
                if *value == 0 {
                    0.0
                } else {
                    let (mantissa, exponent) = value
                        .sci_mantissa_and_exponent_with_rounding(rm)
                        .expect("Value cannot be represented exactly as a float");
                    if let Some(f) =
                        $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
                    {
                        f
                    } else {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as an {}", $f::NAME)
                            }
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                $f::MAX_FINITE
                            }
                            _ => $f::POSITIVE_INFINITY,
                        }
                    }
                }
            }
        }

        impl<'a> From<&'a Natural> for $f {
            /// Converts a [`Natural`] to a primitive float.
            ///
            /// If there are two nearest floats, the one whose least-significant bit is zero is
            /// chosen. If the [`Natural`] is larger than the maximum finite float, then the result
            /// is the maximum finite float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_natural#from).
            #[inline]
            fn from(value: &'a Natural) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $f {
            /// Converts a [`Natural`] to a primitive float.
            ///
            /// If the input isn't exactly equal to some float, `None` is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_natural#checked_from).
            fn checked_from(value: &'a Natural) -> Option<$f> {
                if *value == 0 {
                    Some(0.0)
                } else {
                    let (mantissa, exponent) =
                        value.sci_mantissa_and_exponent_with_rounding(RoundingMode::Exact)?;
                    $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
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
                    if let Some((mantissa, exponent)) =
                        value.sci_mantissa_and_exponent_with_rounding::<$f>(RoundingMode::Exact)
                    {
                        let exponent = i64::exact_from(exponent);
                        if exponent < $f::MIN_EXPONENT || exponent > $f::MAX_EXPONENT {
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
