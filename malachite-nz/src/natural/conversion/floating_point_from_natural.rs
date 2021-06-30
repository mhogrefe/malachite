use malachite_base::named::Named;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom, SciMantissaAndExponent,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        impl<'a> RoundingFrom<&'a Natural> for $f {
            /// Converts a `Natural` to an `f32` or an `f64`, using the specified rounding mode. The
            /// `Natural` is taken by reference. If the input is larger than the maximum finite
            /// value representable by the floating-point type, the result depends on the rounding
            /// mode. If the rounding mode is `Ceiling` or `Up`, the result is positive infinity; if
            /// it is `Exact`, the function panics; otherwise, the result is the maximum finite
            /// float.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::RoundingFrom;
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_nz::natural::Natural;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::rounding_from(&Natural::from_str("123").unwrap(),
            ///     RoundingMode::Exact), 123.0);
            /// assert_eq!(f32::rounding_from(&Natural::from_str("1000000001").unwrap(),
            ///     RoundingMode::Floor), 1.0e9);
            /// assert_eq!(f32::rounding_from(&Natural::from_str("1000000001").unwrap(),
            ///     RoundingMode::Ceiling), 1.00000006e9);
            /// assert_eq!(f32::rounding_from(
            ///     &Natural::from_str("10000000000000000000000000000000000000000000000000000")
            ///     .unwrap(), RoundingMode::Nearest), 3.4028235e38);
            /// ```
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
            /// Converts a `Natural` to the nearest `f32` or an `f64`. The `Natural` is taken by
            /// reference. If there are two nearest floats, the one whose least-significant bit is
            /// zero is chosen. If the input is larger than the maximum finite value representable
            /// by the floating-point type, the result is the maximum finite float.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_nz;
            ///
            /// use malachite_nz::natural::Natural;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::from(&Natural::from_str("123").unwrap()), 123.0);
            /// assert_eq!(f32::from(&Natural::from_str("1000000001").unwrap()), 1.0e9);
            /// assert_eq!(f32::from(
            ///     &Natural::from_str("10000000000000000000000000000000000000000000000000000")
            ///     .unwrap()), 3.4028235e38);
            /// ```
            #[inline]
            fn from(value: &'a Natural) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl<'a> CheckedFrom<&'a Natural> for $f {
            /// Converts a `Natural` to an `f32` or an `f64`. The `Natural` is taken by value. If
            /// the input isn't exactly equal to some float, `None` is returned.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::checked_from(&Natural::from_str("123").unwrap()), Some(123.0));
            /// assert_eq!(f32::checked_from(&Natural::from_str("1000000000").unwrap()),
            ///     Some(1.0e9));
            /// assert_eq!(f32::checked_from(&Natural::from_str("1000000001").unwrap()), None);
            /// assert_eq!(f32::checked_from(
            ///     &Natural::from_str("10000000000000000000000000000000000000000000000000000")
            ///     .unwrap()), None);
            /// ```
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
            /// Determines whether a `Natural` can be exactly converted to an `f32` or `f64`. The
            /// `Natural` is taken by reference.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::convertible_from(&Natural::from_str("123").unwrap()), true);
            /// assert_eq!(f32::convertible_from(&Natural::from_str("1000000000").unwrap()), true);
            /// assert_eq!(f32::convertible_from(&Natural::from_str("1000000001").unwrap()), false);
            /// assert_eq!(f32::convertible_from(
            ///     &Natural::from_str("10000000000000000000000000000000000000000000000000000")
            ///     .unwrap()), false);
            /// ```
            fn convertible_from(value: &'a Natural) -> bool {
                //TODO
                $f::checked_from(value).is_some()
            }
        }
    };
}
float_impls!(f32);
float_impls!(f64);
