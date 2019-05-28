use std::cmp::Ordering;

use malachite_base::conversion::{CheckedFrom, ConvertibleFrom, RoundingFrom, WrappingFrom};
use malachite_base::named::Named;
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::traits::{
    BitAccess, DivisibleByPowerOfTwo, FloorLogTwo, ShrRound, ShrRoundAssign,
};
use malachite_base::round::RoundingMode;

use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::logic::bit_scan::limbs_index_of_next_false_bit;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::Natural::{self, Large, Small};
use platform::Limb;

macro_rules! float_impls {
    ($f: ident, $gt_max_finite_float: ident) => {
        // Returns whether `n` > `$f::MAX_FINITE`.
        fn $gt_max_finite_float(n: &Natural) -> bool {
            match *n {
                Small(_) => false,
                Large(ref limbs) => {
                    const MAX_WIDTH: u64 = $f::MAX_EXPONENT as u64 + 1;
                    match limbs_significant_bits(limbs).cmp(&MAX_WIDTH) {
                        Ordering::Less => false,
                        Ordering::Greater => true,
                        Ordering::Equal => {
                            const TRAILING_ZEROS_OF_MAX: u64 =
                                ($f::MAX_EXPONENT - $f::MANTISSA_WIDTH) as u64;
                            limbs_index_of_next_false_bit(limbs, TRAILING_ZEROS_OF_MAX) >= MAX_WIDTH
                                && !limbs_divisible_by_power_of_two(limbs, TRAILING_ZEROS_OF_MAX)
                        }
                    }
                }
            }
        }

        /// Converts a `Natural` to an `f32` or an `f64`, using the specified rounding mode. The
        /// `Natural` is taken by value. If the input is larger than the maximum finite value
        /// representable by the floating-point type, the result depends on the rounding mode. If
        /// the rounding mode is `Ceiling` or `Up`, the result is positive infinity; if it is
        /// `Exact`, the function panics; otherwise, the result is the maximum finite float.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::conversion::RoundingFrom;
        /// use malachite_base::round::RoundingMode;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::rounding_from(Natural::from_str("123").unwrap(),
        ///         RoundingMode::Exact), 123.0);
        ///     assert_eq!(f32::rounding_from(Natural::from_str("1000000001").unwrap(),
        ///         RoundingMode::Floor), 1.0e9);
        ///     assert_eq!(f32::rounding_from(Natural::from_str("1000000001").unwrap(),
        ///         RoundingMode::Ceiling), 1.00000006e9);
        ///     assert_eq!(f32::rounding_from(
        ///         Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap(), RoundingMode::Nearest), 3.4028235e38);
        /// }
        /// ```
        impl RoundingFrom<Natural> for $f {
            fn rounding_from(mut value: Natural, rm: RoundingMode) -> $f {
                if value == 0 as Limb {
                    return 0.0;
                }
                if $gt_max_finite_float(&value) {
                    return match rm {
                        RoundingMode::Exact => {
                            panic!("Value cannot be represented exactly as an {}", $f::NAME)
                        }
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            $f::MAX_FINITE
                        }
                        _ => $f::POSITIVE_INFINITY,
                    };
                }
                let mut exponent = value.floor_log_two();
                let shift = i32::checked_from(exponent).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                value.shr_round_assign(shift, rm);
                let mut mantissa =
                    <$f as PrimitiveFloat>::UnsignedOfEqualWidth::checked_from(value).unwrap();
                if mantissa.get_bit(u64::from($f::MANTISSA_WIDTH + 1)) {
                    exponent += 1;
                    mantissa >>= 1;
                }
                mantissa.clear_bit(u64::from($f::MANTISSA_WIDTH));
                let exponent = u32::wrapping_from(exponent) + $f::MAX_EXPONENT;
                $f::from_adjusted_mantissa_and_exponent(mantissa, exponent)
            }
        }

        /// Converts a `Natural` to an `f32` or an `f64`, using the specified rounding mode. The
        /// `Natural` is taken by reference. If the input is larger than the maximum finite value
        /// representable by the floating-point type, the result depends on the rounding mode. If
        /// the rounding mode is `Ceiling` or `Up`, the result is positive infinity; if it is
        /// `Exact`, the function panics; otherwise, the result is the maximum finite float.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::conversion::RoundingFrom;
        /// use malachite_base::round::RoundingMode;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::rounding_from(&Natural::from_str("123").unwrap(),
        ///         RoundingMode::Exact), 123.0);
        ///     assert_eq!(f32::rounding_from(&Natural::from_str("1000000001").unwrap(),
        ///         RoundingMode::Floor), 1.0e9);
        ///     assert_eq!(f32::rounding_from(&Natural::from_str("1000000001").unwrap(),
        ///         RoundingMode::Ceiling), 1.00000006e9);
        ///     assert_eq!(f32::rounding_from(
        ///         &Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap(), RoundingMode::Nearest), 3.4028235e38);
        /// }
        /// ```
        impl<'a> RoundingFrom<&'a Natural> for $f {
            fn rounding_from(value: &'a Natural, rm: RoundingMode) -> $f {
                if *value == 0 as Limb {
                    return 0.0;
                }
                if $gt_max_finite_float(value) {
                    return match rm {
                        RoundingMode::Exact => {
                            panic!("Value cannot be represented exactly as an {}", $f::NAME)
                        }
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            $f::MAX_FINITE
                        }
                        _ => $f::POSITIVE_INFINITY,
                    };
                }
                let mut exponent = value.floor_log_two();
                let shift = i32::checked_from(exponent).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                let mut mantissa = <$f as PrimitiveFloat>::UnsignedOfEqualWidth::checked_from(
                    value.shr_round(shift, rm),
                )
                .unwrap();
                if mantissa.get_bit(u64::from($f::MANTISSA_WIDTH + 1)) {
                    exponent += 1;
                    mantissa >>= 1;
                }
                mantissa.clear_bit(u64::from($f::MANTISSA_WIDTH));
                let exponent = u32::wrapping_from(exponent) + $f::MAX_EXPONENT;
                $f::from_adjusted_mantissa_and_exponent(mantissa, exponent)
            }
        }

        /// Converts a `Natural` to the nearest `f32` or an `f64`. The `Natural` is taken by value.
        /// If there are two nearest floats, the one whose least-significant bit is zero is chosen.
        /// If the input is larger than the maximum finite value representable by the floating-point
        /// type, the result is the maximum finite float.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// extern crate malachite_nz;
        ///
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::from(Natural::from_str("123").unwrap()), 123.0);
        ///     assert_eq!(f32::from(Natural::from_str("1000000001").unwrap()), 1.0e9);
        ///     assert_eq!(f32::from(
        ///         Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), 3.4028235e38);
        /// }
        /// ```
        impl From<Natural> for $f {
            fn from(value: Natural) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        /// Converts a `Natural` to the nearest `f32` or an `f64`. The `Natural` is taken by
        /// reference. If there are two nearest floats, the one whose least-significant bit is zero
        /// is chosen. If the input is larger than the maximum finite value representable by the
        /// floating-point type, the result is the maximum finite float.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// extern crate malachite_nz;
        ///
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::from(&Natural::from_str("123").unwrap()), 123.0);
        ///     assert_eq!(f32::from(&Natural::from_str("1000000001").unwrap()), 1.0e9);
        ///     assert_eq!(f32::from(
        ///         &Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), 3.4028235e38);
        /// }
        /// ```
        impl<'a> From<&'a Natural> for $f {
            fn from(value: &'a Natural) -> $f {
                $f::rounding_from(value, RoundingMode::Nearest)
            }
        }

        /// Converts a `Natural` to an `f32` or an `f64`. The `Natural` is taken by value. If the
        /// input isn't exactly equal to some float, `None` is returned.
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
        /// use malachite_base::conversion::CheckedFrom;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::checked_from(Natural::from_str("123").unwrap()), Some(123.0));
        ///     assert_eq!(f32::checked_from(Natural::from_str("1000000000").unwrap()),
        ///         Some(1.0e9));
        ///     assert_eq!(f32::checked_from(Natural::from_str("1000000001").unwrap()), None);
        ///     assert_eq!(f32::checked_from(
        ///         Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), None);
        /// }
        /// ```
        impl CheckedFrom<Natural> for $f {
            fn checked_from(mut value: Natural) -> Option<$f> {
                if value == 0 as Limb {
                    return Some(0.0);
                }
                if $gt_max_finite_float(&value) {
                    return None;
                }
                let exponent = value.floor_log_two();
                let shift = i32::checked_from(exponent).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                if shift >= 0 && !value.divisible_by_power_of_two(u64::wrapping_from(shift)) {
                    return None;
                }
                value >>= shift;
                let mut mantissa =
                    <$f as PrimitiveFloat>::UnsignedOfEqualWidth::wrapping_from(value);
                mantissa.clear_bit(u64::from($f::MANTISSA_WIDTH));
                let exponent = u32::wrapping_from(exponent) + $f::MAX_EXPONENT;
                Some($f::from_adjusted_mantissa_and_exponent(mantissa, exponent))
            }
        }

        /// Converts a `Natural` to an `f32` or an `f64`. The `Natural` is taken by value. If the
        /// input isn't exactly equal to some float, `None` is returned.
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
        /// use malachite_base::conversion::CheckedFrom;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::checked_from(&Natural::from_str("123").unwrap()), Some(123.0));
        ///     assert_eq!(f32::checked_from(&Natural::from_str("1000000000").unwrap()),
        ///         Some(1.0e9));
        ///     assert_eq!(f32::checked_from(&Natural::from_str("1000000001").unwrap()), None);
        ///     assert_eq!(f32::checked_from(
        ///         &Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), None);
        /// }
        /// ```
        impl<'a> CheckedFrom<&'a Natural> for $f {
            fn checked_from(value: &'a Natural) -> Option<$f> {
                if *value == 0 as Limb {
                    return Some(0.0);
                }
                if $gt_max_finite_float(value) {
                    return None;
                }
                let exponent = value.floor_log_two();
                let shift = i32::checked_from(exponent).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                if shift >= 0 && !value.divisible_by_power_of_two(u64::wrapping_from(shift)) {
                    return None;
                }
                let mut mantissa =
                    <$f as PrimitiveFloat>::UnsignedOfEqualWidth::wrapping_from(value >> shift);
                mantissa.clear_bit(u64::from($f::MANTISSA_WIDTH));
                let exponent = u32::wrapping_from(exponent) + $f::MAX_EXPONENT;
                Some($f::from_adjusted_mantissa_and_exponent(mantissa, exponent))
            }
        }

        /// Determines whether a `Natural` can be exactly converted to an `f32` or `f64`. The
        /// `Natural` is taken by value.
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
        /// use malachite_base::conversion::ConvertibleFrom;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::convertible_from(Natural::from_str("123").unwrap()), true);
        ///     assert_eq!(f32::convertible_from(Natural::from_str("1000000000").unwrap()), true);
        ///     assert_eq!(f32::convertible_from(Natural::from_str("1000000001").unwrap()), false);
        ///     assert_eq!(f32::convertible_from(
        ///         Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), false);
        /// }
        /// ```
        impl ConvertibleFrom<Natural> for $f {
            #[inline]
            fn convertible_from(value: Natural) -> bool {
                $f::convertible_from(&value)
            }
        }

        /// Determines whether a `Natural` can be exactly converted to an `f32` or `f64`. The
        /// `Natural` is taken by reference.
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
        /// use malachite_base::conversion::ConvertibleFrom;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::convertible_from(&Natural::from_str("123").unwrap()), true);
        ///     assert_eq!(f32::convertible_from(&Natural::from_str("1000000000").unwrap()), true);
        ///     assert_eq!(f32::convertible_from(&Natural::from_str("1000000001").unwrap()), false);
        ///     assert_eq!(f32::convertible_from(
        ///         &Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap()), false);
        /// }
        /// ```
        impl<'a> ConvertibleFrom<&'a Natural> for $f {
            fn convertible_from(value: &'a Natural) -> bool {
                if *value == 0 as Limb {
                    return true;
                }
                if $gt_max_finite_float(&value) {
                    return false;
                }
                let shift = i32::checked_from(value.floor_log_two()).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                shift < 0 || value.divisible_by_power_of_two(u64::wrapping_from(shift))
            }
        }
    };
}

float_impls!(f32, gt_max_finite_f32);
float_impls!(f64, gt_max_finite_f64);
