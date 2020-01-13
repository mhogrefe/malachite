use malachite_base::num::arithmetic::traits::{DivisibleByPowerOfTwo, ShlRound};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom, WrappingFrom,
};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::round::RoundingMode;

use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        /// Converts an `f32` or an `f64` to a `Natural`, using the specified rounding mode. The
        /// floating-point value cannot be NaN or infinite, and it cannot round to a negative
        /// integer.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `value` is NaN or infinite, if it would round to a negative integer, or if the
        /// rounding mode is `Exact` and `value` is not an integer.
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::conversion::traits::RoundingFrom;
        /// use malachite_base::round::RoundingMode;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(Natural::rounding_from(0.0, RoundingMode::Exact).to_string(), "0");
        /// assert_eq!(Natural::rounding_from(-0.0, RoundingMode::Exact).to_string(), "0");
        /// assert_eq!(Natural::rounding_from(123.0, RoundingMode::Exact).to_string(), "123");
        /// assert_eq!(Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
        ///     "1000000000");
        /// assert_eq!(Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
        ///     "1000000000");
        /// assert_eq!(Natural::rounding_from(4294967295.0, RoundingMode::Exact).to_string(),
        ///     "4294967295");
        /// assert_eq!(Natural::rounding_from(4294967296.0, RoundingMode::Exact).to_string(),
        ///     "4294967296");
        /// assert_eq!(Natural::rounding_from(1.0e100, RoundingMode::Exact).to_string(),
        ///     "1000000000000000015902891109759918046836080856394528138978132755774783877217038106\
        ///      0813469985856815104");
        /// assert_eq!(Natural::rounding_from(123.1, RoundingMode::Floor).to_string(), "123");
        /// assert_eq!(Natural::rounding_from(123.1, RoundingMode::Ceiling).to_string(), "124");
        /// assert_eq!(Natural::rounding_from(123.1, RoundingMode::Nearest).to_string(), "123");
        /// assert_eq!(Natural::rounding_from(123.9, RoundingMode::Floor).to_string(), "123");
        /// assert_eq!(Natural::rounding_from(123.9, RoundingMode::Ceiling).to_string(), "124");
        /// assert_eq!(Natural::rounding_from(123.9, RoundingMode::Nearest).to_string(), "124");
        /// assert_eq!(Natural::rounding_from(123.5, RoundingMode::Nearest).to_string(), "124");
        /// assert_eq!(Natural::rounding_from(124.5, RoundingMode::Nearest).to_string(), "124");
        /// assert_eq!(Natural::rounding_from(-0.99, RoundingMode::Ceiling).to_string(), "0");
        /// assert_eq!(Natural::rounding_from(-0.499, RoundingMode::Nearest).to_string(), "0");
        /// assert_eq!(Natural::rounding_from(-0.5, RoundingMode::Nearest).to_string(), "0");
        /// ```
        impl RoundingFrom<$f> for Natural {
            fn rounding_from(value: $f, rm: RoundingMode) -> Self {
                if value.is_nan() || value.is_infinite() {
                    panic!("Cannot convert {} to Natural", value);
                } else if value == 0.0 {
                    Natural::ZERO
                } else {
                    let (mut mantissa, exponent) = value.to_adjusted_mantissa_and_exponent();
                    let value_negative = value < 0.0;
                    mantissa.set_bit(u64::from($f::MANTISSA_WIDTH));
                    let n = Natural::from(mantissa).shl_round(
                        i32::exact_from(exponent) + $f::MIN_EXPONENT - 1,
                        if value_negative { -rm } else { rm },
                    );
                    if value_negative && n != 0 {
                        panic!("Result is negative and cannot be converted to a Natural");
                    }
                    n
                }
            }
        }

        /// Converts an `f32` or `f64` to the nearest `Natural`. Floating-point values exactly
        /// between two `Natural`s are rounded to the even one. The floating point value cannot be
        /// NaN or infinite, and it cannot round to a negative integer (so it must be greater than
        /// or equal to -0.5).
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `value` is NaN or infinite, or if it would round to a negative integer.
        ///
        /// # Example
        /// ```
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(Natural::from(0.0).to_string(), "0");
        /// assert_eq!(Natural::from(-0.0).to_string(), "0");
        /// assert_eq!(Natural::from(123.0).to_string(), "123");
        /// assert_eq!(Natural::from(1.0e9).to_string(), "1000000000");
        /// assert_eq!(Natural::from(4294967295.0).to_string(), "4294967295");
        /// assert_eq!(Natural::from(4294967296.0).to_string(), "4294967296");
        /// assert_eq!(Natural::from(1.0e100).to_string(),
        ///         "100000000000000001590289110975991804683608085639452813897813275577478387721703\
        ///         81060813469985856815104");
        /// assert_eq!(Natural::from(123.1).to_string(), "123");
        /// assert_eq!(Natural::from(123.9).to_string(), "124");
        /// assert_eq!(Natural::from(123.5).to_string(), "124");
        /// assert_eq!(Natural::from(124.5).to_string(), "124");
        /// assert_eq!(Natural::from(-0.499).to_string(), "0");
        /// assert_eq!(Natural::from(-0.5).to_string(), "0");
        /// ```
        impl From<$f> for Natural {
            fn from(value: $f) -> Natural {
                Natural::rounding_from(value, RoundingMode::Nearest)
            }
        }

        /// If an `f32` or `f64` is exactly equal to a `Natural`, returns the `Natural`. Otherwise,
        /// returns `None`.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::conversion::traits::CheckedFrom;
        /// use malachite_base::num::floats::PrimitiveFloat;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(format!("{:?}", Natural::checked_from(f64::NAN)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(f64::POSITIVE_INFINITY)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(f64::NEGATIVE_INFINITY)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(0.0)), "Some(0)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(-0.0)), "Some(0)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(123.0)), "Some(123)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(1.0e9)), "Some(1000000000)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(4294967295.0)),
        ///     "Some(4294967295)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(4294967296.0)),
        ///     "Some(4294967296)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(1.0e100)),
        ///     "Some(10000000000000000159028911097599180468360808563945281389781327557747838772170\
        ///      381060813469985856815104)");
        /// assert_eq!(format!("{:?}", Natural::checked_from(123.1)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(123.9)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(123.5)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(124.5)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(-0.499)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(-0.5)), "None");
        /// assert_eq!(format!("{:?}", Natural::checked_from(-123.0)), "None");
        /// ```
        impl CheckedFrom<$f> for Natural {
            fn checked_from(value: $f) -> Option<Natural> {
                if value.is_nan() || value.is_infinite() || value < 0.0 {
                    None
                } else if value == 0.0 {
                    Some(Natural::ZERO)
                } else {
                    let (mut mantissa, exponent) = value.to_adjusted_mantissa_and_exponent();
                    mantissa.set_bit(u64::from($f::MANTISSA_WIDTH));
                    let exponent = i32::exact_from(exponent) + $f::MIN_EXPONENT - 1;
                    if exponent >= 0
                        || mantissa.divisible_by_power_of_two(u64::wrapping_from(-exponent))
                    {
                        Some(Natural::from(mantissa) << exponent)
                    } else {
                        None
                    }
                }
            }
        }

        /// Determines whether an `f32` or `f64` can be exactly converted to a `Natural`.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::conversion::traits::ConvertibleFrom;
        /// use malachite_base::num::floats::PrimitiveFloat;
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
        impl ConvertibleFrom<$f> for Natural {
            fn convertible_from(value: $f) -> bool {
                if value.is_nan() || value.is_infinite() || value < 0.0 {
                    false
                } else if value == 0.0 {
                    true
                } else {
                    let (mut mantissa, exponent) = value.to_adjusted_mantissa_and_exponent();
                    mantissa.set_bit(u64::from($f::MANTISSA_WIDTH));
                    let exponent = i32::exact_from(exponent) + $f::MIN_EXPONENT - 1;
                    exponent >= 0
                        || mantissa.divisible_by_power_of_two(u64::wrapping_from(-exponent))
                }
            }
        }
    };
}

float_impls!(f32);
float_impls!(f64);
