use malachite_base::num::arithmetic::traits::ShlRound;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, IntegerMantissaAndExponent, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<$f> for Natural {
            /// Converts an `f32` or an `f64` to a `Natural`, using the specified rounding mode. The
            /// floating-point value cannot be NaN or infinite, and it cannot round to a negative
            /// integer.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite, if it would round to a negative integer, or if
            /// the rounding mode is `Exact` and `value` is not an integer.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::RoundingFrom;
            /// use malachite_base::rounding_modes::RoundingMode;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     Natural::rounding_from(0.0, RoundingMode::Exact).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(-0.0, RoundingMode::Exact).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.0, RoundingMode::Exact).to_string(),
            ///     "123"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
            ///     "1000000000"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
            ///     "1000000000"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(4294967295.0, RoundingMode::Exact).to_string(),
            ///     "4294967295"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(4294967296.0, RoundingMode::Exact).to_string(),
            ///     "4294967296"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(1.0e100, RoundingMode::Exact).to_string(),
            ///     "100000000000000001590289110975991804683608085639452813897813275577478387721703\
            ///     81060813469985856815104"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.1, RoundingMode::Floor).to_string(),
            ///     "123"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.1, RoundingMode::Ceiling).to_string(),
            ///     "124"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.1, RoundingMode::Nearest).to_string(),
            ///     "123"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.9, RoundingMode::Floor).to_string(),
            ///     "123"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.9, RoundingMode::Ceiling).to_string(),
            ///     "124"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.9, RoundingMode::Nearest).to_string(),
            ///     "124"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(123.5, RoundingMode::Nearest).to_string(),
            ///     "124"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(124.5, RoundingMode::Nearest).to_string(),
            ///     "124"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(-0.99, RoundingMode::Ceiling).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(-0.499, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// assert_eq!(
            ///     Natural::rounding_from(-0.5, RoundingMode::Nearest).to_string(),
            ///     "0"
            /// );
            /// ```
            fn rounding_from(value: $f, rm: RoundingMode) -> Self {
                if value.is_nan() || value == $f::POSITIVE_INFINITY {
                    panic!("Cannot convert {} to Natural", value);
                } else if value == 0.0 {
                    Natural::ZERO
                } else if value < 0.0 {
                    if rm == RoundingMode::Down
                        || rm == RoundingMode::Ceiling
                        || rm == RoundingMode::Nearest
                    {
                        Natural::ZERO
                    } else {
                        panic!("Result is negative and cannot be converted to a Natural");
                    }
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    Natural::from(mantissa).shl_round(exponent, rm)
                }
            }
        }

        impl From<$f> for Natural {
            /// Converts an `f32` or `f64` to the nearest `Natural`. Floating-point values exactly
            /// between two `Natural`s are rounded to the even one. The floating point value cannot
            /// be NaN or infinite, and it cannot round to a negative integer (so it must be greater
            /// than or equal to -0.5).
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite, or if it would round to a negative integer.
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(0.0).to_string(), "0");
            /// assert_eq!(Natural::from(-0.0).to_string(), "0");
            /// assert_eq!(Natural::from(123.0).to_string(), "123");
            /// assert_eq!(Natural::from(1.0e9).to_string(), "1000000000");
            /// assert_eq!(Natural::from(4294967295.0).to_string(), "4294967295");
            /// assert_eq!(Natural::from(4294967296.0).to_string(), "4294967296");
            /// assert_eq!(
            ///     Natural::from(1.0e100).to_string(),
            ///     "100000000000000001590289110975991804683608085639452813897813275577478387721703\
            ///     81060813469985856815104"
            /// );
            /// assert_eq!(Natural::from(123.1).to_string(), "123");
            /// assert_eq!(Natural::from(123.9).to_string(), "124");
            /// assert_eq!(Natural::from(123.5).to_string(), "124");
            /// assert_eq!(Natural::from(124.5).to_string(), "124");
            /// assert_eq!(Natural::from(-0.499).to_string(), "0");
            /// assert_eq!(Natural::from(-0.5).to_string(), "0");
            /// ```
            fn from(value: $f) -> Natural {
                Natural::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl CheckedFrom<$f> for Natural {
            /// If an `f32` or `f64` is exactly equal to a `Natural`, returns the `Natural`.
            /// Otherwise, returns `None`.
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
            /// use malachite_base::num::basic::floats::PrimitiveFloat;
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(format!("{:?}", Natural::checked_from(f64::NAN)), "None");
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(f64::POSITIVE_INFINITY)),
            ///     "None"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(f64::NEGATIVE_INFINITY)),
            ///     "None"
            /// );
            /// assert_eq!(format!("{:?}", Natural::checked_from(0.0)), "Some(0)");
            /// assert_eq!(format!("{:?}", Natural::checked_from(-0.0)), "Some(0)");
            /// assert_eq!(format!("{:?}", Natural::checked_from(123.0)), "Some(123)");
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(1.0e9)),
            ///     "Some(1000000000)"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(4294967295.0)),
            ///     "Some(4294967295)"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(4294967296.0)),
            ///     "Some(4294967296)"
            /// );
            /// assert_eq!(
            ///     format!("{:?}", Natural::checked_from(1.0e100)),
            ///     "Some(1000000000000000015902891109759918046836080856394528138978132755774783877\
            ///     2170381060813469985856815104)"
            /// );
            /// assert_eq!(format!("{:?}", Natural::checked_from(123.1)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(123.9)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(123.5)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(124.5)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(-0.499)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(-0.5)), "None");
            /// assert_eq!(format!("{:?}", Natural::checked_from(-123.0)), "None");
            /// ```
            fn checked_from(value: $f) -> Option<Natural> {
                if value.is_nan() || value.is_infinite() || value < 0.0 {
                    None
                } else if value == 0.0 {
                    Some(Natural::ZERO)
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    if exponent >= 0 {
                        Some(Natural::from(mantissa) << exponent)
                    } else {
                        None
                    }
                }
            }
        }

        impl ConvertibleFrom<$f> for Natural {
            /// Determines whether an `f32` or `f64` can be exactly converted to a `Natural`.
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
            /// use malachite_base::num::basic::floats::PrimitiveFloat;
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::convertible_from(f64::NAN), false);
            /// assert_eq!(Natural::convertible_from(f64::POSITIVE_INFINITY), false);
            /// assert_eq!(Natural::convertible_from(f64::NEGATIVE_INFINITY), false);
            /// assert_eq!(Natural::convertible_from(0.0), true);
            /// assert_eq!(Natural::convertible_from(-0.0), true);
            /// assert_eq!(Natural::convertible_from(123.0), true);
            /// assert_eq!(Natural::convertible_from(1.0e9), true);
            /// assert_eq!(Natural::convertible_from(4294967295.0), true);
            /// assert_eq!(Natural::convertible_from(4294967296.0), true);
            /// assert_eq!(Natural::convertible_from(1.0e100), true);
            /// assert_eq!(Natural::convertible_from(123.1), false);
            /// assert_eq!(Natural::convertible_from(123.9), false);
            /// assert_eq!(Natural::convertible_from(123.5), false);
            /// assert_eq!(Natural::convertible_from(124.5), false);
            /// assert_eq!(Natural::convertible_from(-0.499), false);
            /// assert_eq!(Natural::convertible_from(-0.5), false);
            /// assert_eq!(Natural::convertible_from(-123.0), false);
            /// ```
            #[inline]
            fn convertible_from(value: $f) -> bool {
                value >= 0.0 && value.is_integer()
            }
        }
    };
}
float_impls!(f32);
float_impls!(f64);
