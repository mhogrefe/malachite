use malachite_base::num::arithmetic::traits::{
    DivRound, DivisibleByPowerOf2, IsPowerOf2, NegAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RawMantissaAndExponent, RoundingFrom,
    SciMantissaAndExponent, WrappingFrom,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use Rational;

fn abs_is_neg_power_of_2(x: &Rational) -> bool {
    x.numerator == 1u32 && x.denominator.is_power_of_2()
}

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<Rational> for $f {
            /// Converts a `Rational` to an `f32` or an `f64`, using the specified rounding mode.
            /// The `Rational` is taken by value.
            ///
            /// If the input is larger than the maximum finite value representable by the floating-
            /// point type, the result depends on the rounding mode. If the rounding mode is
            /// `Ceiling` or `Up`, the result is positive infinity; if it is `Exact`, the function
            /// panics; otherwise, the result is the maximum finite float.
            ///
            /// If the input is smaller than the minimum (most negative) finite value, this
            /// function's behavior is similar. If the rounding mode is `Floor` or `Up`, the result
            /// is negative infinity; if it is `Exact`, the function panics; otherwise, the result
            /// is the minimum finite float.
            ///
            /// If the absolute value of the `Rational` is less than the smallest positive float,
            /// and the rounding mode is towards zero, then the result will be positive or negative
            /// zero, depending on the sign of the `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn rounding_from(mut value: Rational, mut rm: RoundingMode) -> $f {
                if value == 0u32 {
                    0.0
                } else {
                    let sign = value.sign;
                    if !sign {
                        rm.neg_assign();
                    }
                    let mut exponent = value.floor_log_base_2_of_abs();
                    let f = if exponent > $f::MAX_EXPONENT {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                $f::MAX_FINITE
                            }
                            _ => $f::POSITIVE_INFINITY,
                        }
                    } else if exponent >= $f::MIN_NORMAL_EXPONENT {
                        value >>= exponent - i64::wrapping_from($f::MANTISSA_WIDTH);
                        let (n, d) = value.into_numerator_and_denominator();
                        let mut mantissa = n.div_round(d, rm);
                        let mut bits = mantissa.significant_bits();
                        let mut done = false;
                        if bits > $f::MANTISSA_WIDTH + 1 {
                            if exponent == $f::MAX_EXPONENT {
                                done = true;
                            } else {
                                bits -= 1;
                                mantissa >>= 1; // lsb is zero
                                exponent += 1;
                            }
                        }
                        if done {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a float")
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => $f::MAX_FINITE,
                                _ => $f::POSITIVE_INFINITY,
                            }
                        } else {
                            assert_eq!(bits, $f::MANTISSA_WIDTH + 1);
                            mantissa.clear_bit($f::MANTISSA_WIDTH);
                            $f::from_raw_mantissa_and_exponent(
                                u64::exact_from(&mantissa),
                                u64::exact_from(exponent + $f::MAX_EXPONENT),
                            )
                        }
                    } else if exponent >= $f::MIN_EXPONENT {
                        let target_width = u64::wrapping_from(exponent - $f::MIN_EXPONENT + 1);
                        value >>= $f::MIN_EXPONENT;
                        let (n, d) = value.into_numerator_and_denominator();
                        let mantissa = n.div_round(d, rm);
                        if mantissa.significant_bits() > target_width
                            && exponent == $f::MIN_NORMAL_EXPONENT - 1
                        {
                            $f::MIN_POSITIVE_NORMAL
                        } else {
                            $f::from_raw_mantissa_and_exponent(u64::exact_from(&mantissa), 0)
                        }
                    } else {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down => 0.0,
                            RoundingMode::Nearest => {
                                if exponent == $f::MIN_EXPONENT - 1
                                    && !abs_is_neg_power_of_2(&value)
                                {
                                    $f::MIN_POSITIVE_SUBNORMAL
                                } else {
                                    0.0
                                }
                            }
                            _ => $f::MIN_POSITIVE_SUBNORMAL,
                        }
                    };
                    if sign {
                        f
                    } else {
                        -f
                    }
                }
            }
        }

        impl From<Rational> for $f {
            /// Converts a `Rational` to the nearest `f32` or an `f64`. The `Rational` is taken by
            /// value. If there are two nearest floats, the one whose least-significant bit is zero
            /// is chosen.
            ///
            /// If the input is larger than the maximum finite value representable by the
            /// floating-point type, the result is the maximum finite float. If the input is
            /// smaller than the minimum (most negative) finite value, the result is the minimum
            /// finite float.
            ///
            /// If the absolute value of the `Rational` is half of the smallest positive float or
            /// smaller, zero is returned. The sign of the zero is the same as the sign of the
            /// `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            #[inline]
            fn from(value: Rational) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl CheckedFrom<Rational> for $f {
            /// Converts a `Rational` to an `f32` or an `f64`. The `Rational` is taken by value. If
            /// the input isn't exactly equal to any float, `None` is returned.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn checked_from(value: Rational) -> Option<$f> {
                if value == 0 {
                    Some(0.0)
                } else {
                    let sign = value.sign;
                    let (mantissa, exponent) =
                        value.sci_mantissa_and_exponent_with_rounding(RoundingMode::Exact)?;
                    let f = $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent));
                    if sign {
                        f
                    } else {
                        f.map(|x| -x)
                    }
                }
            }
        }

        impl ConvertibleFrom<Rational> for $f {
            /// Determines whether a `Rational` can be exactly converted to an `f32` or `f64`. The
            /// `Rational` is taken by value.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn convertible_from(value: Rational) -> bool {
                if value == 0 {
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

        impl<'a> RoundingFrom<&'a Rational> for $f {
            /// Converts a `Rational` to an `f32` or an `f64`, using the specified rounding mode.
            /// The `Rational` is taken by reference.
            ///
            /// If the input is larger than the maximum finite value representable by the floating-
            /// point type, the result depends on the rounding mode. If the rounding mode is
            /// `Ceiling` or `Up`, the result is positive infinity; if it is `Exact`, the function
            /// panics; otherwise, the result is the maximum finite float.
            ///
            /// If the input is smaller than the minimum (most negative) finite value, this
            /// function's behavior is similar. If the rounding mode is `Floor` or `Up`, the result
            /// is negative infinity; if it is `Exact`, the function panics; otherwise, the result
            /// is the minimum finite float.
            ///
            /// If the absolute value of the `Rational` is less than the smallest positive float,
            /// and the rounding mode is towards zero, then the result will be positive or negative
            /// zero, depending on the sign of the `Rational`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn rounding_from(value: &'a Rational, mut rm: RoundingMode) -> $f {
                if *value == 0u32 {
                    0.0
                } else {
                    if !value.sign {
                        rm.neg_assign();
                    }
                    let mut exponent = value.floor_log_base_2_of_abs();
                    let f = if exponent > $f::MAX_EXPONENT {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                $f::MAX_FINITE
                            }
                            _ => $f::POSITIVE_INFINITY,
                        }
                    } else if exponent >= $f::MIN_NORMAL_EXPONENT {
                        let x = value >> exponent - i64::wrapping_from($f::MANTISSA_WIDTH);
                        let (n, d) = x.into_numerator_and_denominator();
                        let mut mantissa = n.div_round(d, rm);
                        let mut bits = mantissa.significant_bits();
                        let mut done = false;
                        if bits > $f::MANTISSA_WIDTH + 1 {
                            if exponent == $f::MAX_EXPONENT {
                                done = true;
                            } else {
                                bits -= 1;
                                mantissa >>= 1; // lsb is zero
                                exponent += 1;
                            }
                        }
                        if done {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a float")
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => $f::MAX_FINITE,
                                _ => $f::POSITIVE_INFINITY,
                            }
                        } else {
                            assert_eq!(bits, $f::MANTISSA_WIDTH + 1);
                            mantissa.clear_bit($f::MANTISSA_WIDTH);
                            $f::from_raw_mantissa_and_exponent(
                                u64::exact_from(&mantissa),
                                u64::exact_from(exponent + $f::MAX_EXPONENT),
                            )
                        }
                    } else if exponent >= $f::MIN_EXPONENT {
                        let target_width = u64::wrapping_from(exponent - $f::MIN_EXPONENT + 1);
                        let x = value >> $f::MIN_EXPONENT;
                        let (n, d) = x.into_numerator_and_denominator();
                        let mantissa = n.div_round(d, rm);
                        if mantissa.significant_bits() > target_width
                            && exponent == $f::MIN_NORMAL_EXPONENT - 1
                        {
                            $f::MIN_POSITIVE_NORMAL
                        } else {
                            $f::from_raw_mantissa_and_exponent(u64::exact_from(&mantissa), 0)
                        }
                    } else {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down => 0.0,
                            RoundingMode::Nearest => {
                                if exponent == $f::MIN_EXPONENT - 1
                                    && !abs_is_neg_power_of_2(&value)
                                {
                                    $f::MIN_POSITIVE_SUBNORMAL
                                } else {
                                    0.0
                                }
                            }
                            _ => $f::MIN_POSITIVE_SUBNORMAL,
                        }
                    };
                    if value.sign {
                        f
                    } else {
                        -f
                    }
                }
            }
        }

        impl<'a> From<&'a Rational> for $f {
            /// Converts a `Rational` to the nearest `f32` or an `f64`. The `Rational` is taken by
            /// reference. If there are two nearest floats, the one whose least-significant bit is
            /// zero is chosen.
            ///
            /// If the input is larger than the maximum finite value representable by the
            /// floating-point type, the result is the maximum finite float. If the input is
            /// smaller than the minimum (most negative) finite value, the result is the minimum
            /// finite float.
            ///
            /// If the absolute value of the `Rational` is half of the smallest positive float or
            /// smaller, zero is returned. The sign of the zero is the same as the sign of the
            /// `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            #[inline]
            fn from(value: &'a Rational) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl<'a> CheckedFrom<&'a Rational> for $f {
            /// Converts a `Rational` to an `f32` or an `f64`. The `Rational` is taken by
            /// reference. If the input isn't exactly equal to any float, `None` is returned.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn checked_from(value: &'a Rational) -> Option<$f> {
                if *value == 0 {
                    Some(0.0)
                } else {
                    let (mantissa, exponent) =
                        value.sci_mantissa_and_exponent_with_rounding_ref(RoundingMode::Exact)?;
                    let f = $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent));
                    if value.sign {
                        f
                    } else {
                        f.map(|x| -x)
                    }
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Rational> for $f {
            /// Determines whether a `Rational` can be exactly converted to an `f32` or `f64`. The
            /// `Rational` is taken by reference.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `conversion::floating_point_from_rational` module.
            fn convertible_from(value: &'a Rational) -> bool {
                if *value == 0 {
                    true
                } else {
                    if let Some((mantissa, exponent)) =
                        value.sci_mantissa_and_exponent_with_rounding_ref::<$f>(RoundingMode::Exact)
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
float_impls!(f32);
float_impls!(f64);
