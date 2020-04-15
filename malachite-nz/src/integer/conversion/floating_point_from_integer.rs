use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::round::RoundingMode;

use integer::Integer;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<Integer> for $f {
            /// Converts an `Integer` to an `f32` or an `f64`, using the specified rounding mode.
            /// The `Integer` is taken by value.
            ///
            /// If the input is larger than the maximum finite value representable by the
            /// floating-point type, the result depends on the rounding mode. If the rounding mode
            /// is `Ceiling` or `Up`, the result is positive infinity; if it is `Exact`, the
            /// function panics; otherwise, the result is the maximum finite float.
            ///
            /// If the input is smaller than the minimum (most negative) finite value, this
            /// function's behavior is similar. If the rounding mode is `Floor` or `Up`, the result
            /// is negative infinity; if it is `Exact`, the function panics; otherwise, the result
            /// is the minimum finite float.
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
            /// use malachite_base::num::conversion::traits::RoundingFrom;
            /// use malachite_base::round::RoundingMode;
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::rounding_from(Integer::from_str("123").unwrap(),
            ///     RoundingMode::Exact), 123.0);
            /// assert_eq!(f32::rounding_from(Integer::from_str("1000000001").unwrap(),
            ///     RoundingMode::Floor), 1.0e9);
            /// assert_eq!(f32::rounding_from(Integer::from_str("1000000001").unwrap(),
            ///     RoundingMode::Ceiling), 1.00000006e9);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("-1000000001").unwrap(),
            ///     RoundingMode::Floor), -1.00000006e9);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("-1000000001").unwrap(),
            ///     RoundingMode::Ceiling), -1.0e9);
            /// assert_eq!(
            ///     f32::rounding_from(
            ///         Integer::from_str("10000000000000000000000000000000000000000000000000000")
            ///             .unwrap(),
            ///         RoundingMode::Nearest
            ///     ),
            ///     3.4028235e38
            /// );
            /// ```
            fn rounding_from(value: Integer, rm: RoundingMode) -> $f {
                if value.sign {
                    $f::rounding_from(value.abs, rm)
                } else {
                    -$f::rounding_from(value.abs, -rm)
                }
            }
        }

        impl<'a> RoundingFrom<&'a Integer> for $f {
            /// Converts an `Integer` to an `f32` or an `f64`, using the specified rounding mode.
            /// The `Integer` is taken by reference.
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
            /// use malachite_base::num::conversion::traits::RoundingFrom;
            /// use malachite_base::round::RoundingMode;
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::rounding_from(&Integer::from_str("123").unwrap(),
            ///     RoundingMode::Exact), 123.0);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("1000000001").unwrap(),
            ///     RoundingMode::Floor), 1.0e9);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("1000000001").unwrap(),
            ///     RoundingMode::Ceiling), 1.00000006e9);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("-1000000001").unwrap(),
            ///     RoundingMode::Floor), -1.00000006e9);
            /// assert_eq!(f32::rounding_from(&Integer::from_str("-1000000001").unwrap(),
            ///     RoundingMode::Ceiling), -1.0e9);
            /// assert_eq!(f32::rounding_from(
            ///     &Integer::from_str("10000000000000000000000000000000000000000000000000000")
            ///     .unwrap(), RoundingMode::Nearest), 3.4028235e38);
            /// ```
            fn rounding_from(value: &'a Integer, rm: RoundingMode) -> $f {
                if value.sign {
                    $f::rounding_from(&value.abs, rm)
                } else {
                    -$f::rounding_from(&value.abs, -rm)
                }
            }
        }

        impl From<Integer> for $f {
            /// Converts an `Integer` to the nearest `f32` or an `f64`. The `Integer` is taken by
            /// value. If there are two nearest floats, the one whose least-significant bit is zero
            /// is chosen. If the input is larger than the maximum finite value representable by the
            /// floating-point type, the result is the maximum finite float. If the input is smaller
            /// than the minimum (most negative) finite value, the result is the minimum finite
            /// float.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_nz;
            ///
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::from(Integer::from_str("123").unwrap()), 123.0);
            /// assert_eq!(f32::from(Integer::from_str("-1000000001").unwrap()), -1.0e9);
            /// assert_eq!(
            ///     f32::from(
            ///         Integer::from_str("10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     3.4028235e38
            /// );
            /// ```
            fn from(value: Integer) -> $f {
                let abs = $f::from(value.abs);
                if value.sign {
                    abs
                } else {
                    -abs
                }
            }
        }

        impl<'a> From<&'a Integer> for $f {
            /// Converts an `Integer` to the nearest `f32` or an `f64`. The `Integer` is taken by
            /// reference. If there are two nearest floats, the one whose least-significant bit is
            /// zero is chosen. If the input is larger than the maximum finite value representable
            /// by the floating-point type, the result is the maximum finite float. If the input is
            /// smaller than the minimum (most negative) finite value, the result is the minimum
            /// finite float.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// extern crate malachite_nz;
            ///
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::from(&Integer::from_str("123").unwrap()), 123.0);
            /// assert_eq!(f32::from(&Integer::from_str("-1000000001").unwrap()), -1.0e9);
            /// assert_eq!(
            ///     f32::from(
            ///         &Integer::from_str("10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     3.4028235e38
            /// );
            /// ```
            fn from(value: &'a Integer) -> $f {
                let abs = $f::from(&value.abs);
                if value.sign {
                    abs
                } else {
                    -abs
                }
            }
        }

        impl CheckedFrom<Integer> for $f {
            /// Converts an `Integer` to an `f32` or an `f64`. The `Integer` is taken by value. If
            /// the input isn't exactly equal to some float, `None` is returned.
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
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::checked_from(Integer::from_str("123").unwrap()), Some(123.0));
            /// assert_eq!(
            ///     f32::checked_from(Integer::from_str("-1000000000").unwrap()),
            ///     Some(-1.0e9)
            /// );
            /// assert_eq!(f32::checked_from(Integer::from_str("1000000001").unwrap()), None);
            /// assert_eq!(
            ///     f32::checked_from(
            ///         Integer::from_str("-10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     None
            /// );
            /// ```
            fn checked_from(value: Integer) -> Option<$f> {
                let sign = value.sign;
                $f::checked_from(value.abs).map(|f| if sign { f } else { -f })
            }
        }

        impl<'a> CheckedFrom<&'a Integer> for $f {
            /// Converts an `Integer` to an `f32` or an `f64`. The `Integer` is taken by value. If
            /// the input isn't exactly equal to some float, `None` is returned.
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
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::checked_from(&Integer::from_str("123").unwrap()), Some(123.0));
            /// assert_eq!(f32::checked_from(&Integer::from_str("-1000000000").unwrap()),
            ///     Some(-1.0e9));
            /// assert_eq!(f32::checked_from(&Integer::from_str("1000000001").unwrap()), None);
            /// assert_eq!(
            ///     f32::checked_from(
            ///         &Integer::from_str("-10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     None
            /// );
            /// ```
            fn checked_from(value: &'a Integer) -> Option<$f> {
                $f::checked_from(&value.abs).map(|f| if value.sign { f } else { -f })
            }
        }

        impl ConvertibleFrom<Integer> for $f {
            /// Determines whether an `Integer` can be exactly converted to an `f32` or `f64`. The
            /// `Integer` is taken by value.
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
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::convertible_from(Integer::from_str("123").unwrap()), true);
            /// assert_eq!(f32::convertible_from(Integer::from_str("-1000000000").unwrap()), true);
            /// assert_eq!(f32::convertible_from(Integer::from_str("1000000001").unwrap()), false);
            /// assert_eq!(
            ///     f32::convertible_from(
            ///         Integer::from_str("-10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     false
            /// );
            /// ```
            #[inline]
            fn convertible_from(value: Integer) -> bool {
                $f::convertible_from(&value)
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $f {
            /// Determines whether an `Integer` can be exactly converted to an `f32` or `f64`. The
            /// `Integer` is taken by reference.
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
            /// use malachite_nz::integer::Integer;
            /// use std::str::FromStr;
            ///
            /// assert_eq!(f32::convertible_from(&Integer::from_str("123").unwrap()), true);
            /// assert_eq!(f32::convertible_from(&Integer::from_str("-1000000000").unwrap()), true);
            /// assert_eq!(f32::convertible_from(&Integer::from_str("1000000001").unwrap()), false);
            /// assert_eq!(
            ///     f32::convertible_from(
            ///         &Integer::from_str("-10000000000000000000000000000000000000000000000000000")
            ///             .unwrap()
            ///     ),
            ///     false
            /// );
            /// ```
            fn convertible_from(value: &'a Integer) -> bool {
                $f::convertible_from(&value.abs)
            }
        }
    };
}

float_impls!(f32);
float_impls!(f64);
