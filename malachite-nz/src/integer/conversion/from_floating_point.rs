use integer::Integer;
use malachite_base::misc::{CheckedFrom, RoundingFrom};
use malachite_base::round::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        /// Converts an `f32` or an `f64` to an `Integer`, using the specified rounding mode. The
        /// floating-point value cannot be NaN or infinite.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `value` is NaN or infinite or if the rounding mode is `Exact` and `value` is
        /// not an integer.
        ///
        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::misc::RoundingFrom;
        /// use malachite_base::round::RoundingMode;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!(Integer::rounding_from(0.0, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!(Integer::rounding_from(-0.0, RoundingMode::Exact).to_string(), "0");
        ///     assert_eq!(Integer::rounding_from(123.0, RoundingMode::Exact).to_string(), "123");
        ///     assert_eq!(Integer::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
        ///         "1000000000");
        ///     assert_eq!(Integer::rounding_from(1.0e9, RoundingMode::Exact).to_string(),
        ///         "1000000000");
        ///     assert_eq!(Integer::rounding_from(4294967295.0, RoundingMode::Exact).to_string(),
        ///         "4294967295");
        ///     assert_eq!(Integer::rounding_from(4294967296.0, RoundingMode::Exact).to_string(),
        ///         "4294967296");
        ///     assert_eq!(Integer::rounding_from(1.0e100, RoundingMode::Exact).to_string(),
        ///         "100000000000000001590289110975991804683608085639452813897813275577478387721703\
        ///         81060813469985856815104");
        ///     assert_eq!(Integer::rounding_from(123.1, RoundingMode::Floor).to_string(), "123");
        ///     assert_eq!(Integer::rounding_from(123.1, RoundingMode::Ceiling).to_string(), "124");
        ///     assert_eq!(Integer::rounding_from(123.1, RoundingMode::Nearest).to_string(), "123");
        ///     assert_eq!(Integer::rounding_from(123.9, RoundingMode::Floor).to_string(), "123");
        ///     assert_eq!(Integer::rounding_from(123.9, RoundingMode::Ceiling).to_string(), "124");
        ///     assert_eq!(Integer::rounding_from(123.9, RoundingMode::Nearest).to_string(), "124");
        ///     assert_eq!(Integer::rounding_from(123.5, RoundingMode::Nearest).to_string(), "124");
        ///     assert_eq!(Integer::rounding_from(124.5, RoundingMode::Nearest).to_string(), "124");
        ///     assert_eq!(Integer::rounding_from(-0.99, RoundingMode::Ceiling).to_string(), "0");
        ///     assert_eq!(Integer::rounding_from(-0.499, RoundingMode::Nearest).to_string(), "0");
        ///     assert_eq!(Integer::rounding_from(-0.5, RoundingMode::Nearest).to_string(), "0");
        /// }
        /// ```
        impl RoundingFrom<$f> for Integer {
            fn rounding_from(value: $f, rm: RoundingMode) -> Self {
                if value >= 0.0 {
                    Integer {
                        sign: true,
                        abs: Natural::rounding_from(value, rm),
                    }
                } else {
                    let abs = Natural::rounding_from(-value, -rm);
                    Integer {
                        sign: abs == 0,
                        abs,
                    }
                }
            }
        }

        /// Converts an `f32` or `f64` to the nearest `Integer`. Floating-point values exactly
        /// between two `Integer`s are rounded to the even one. The floating point value cannot be
        /// NaN or infinite.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Panics
        /// Panics if `value` is NaN or infinite.
        ///
        /// # Example
        /// ```
        /// use malachite_nz::integer::Integer;
        ///
        /// assert_eq!(Integer::from(0.0).to_string(), "0");
        /// assert_eq!(Integer::from(-0.0).to_string(), "0");
        /// assert_eq!(Integer::from(123.0).to_string(), "123");
        /// assert_eq!(Integer::from(1.0e9).to_string(), "1000000000");
        /// assert_eq!(Integer::from(4294967295.0).to_string(), "4294967295");
        /// assert_eq!(Integer::from(4294967296.0).to_string(), "4294967296");
        /// assert_eq!(Integer::from(1.0e100).to_string(),
        ///         "100000000000000001590289110975991804683608085639452813897813275577478387721703\
        ///         81060813469985856815104");
        /// assert_eq!(Integer::from(123.1).to_string(), "123");
        /// assert_eq!(Integer::from(123.9).to_string(), "124");
        /// assert_eq!(Integer::from(123.5).to_string(), "124");
        /// assert_eq!(Integer::from(124.5).to_string(), "124");
        /// assert_eq!(Integer::from(-0.499).to_string(), "0");
        /// assert_eq!(Integer::from(-0.5).to_string(), "0");
        /// ```
        impl From<$f> for Integer {
            fn from(value: $f) -> Integer {
                let abs = Natural::from(value.abs());
                Integer {
                    sign: value >= 0.0 || abs == 0,
                    abs,
                }
            }
        }

        /// If an `f32` or `f64` is exactly equal to an `Integer`, returns the `Integer`. Otherwise,
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
        /// use malachite_base::misc::CheckedFrom;
        /// use malachite_base::num::PrimitiveFloat;
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     assert_eq!(format!("{:?}", Integer::checked_from(f64::NAN)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(f64::POSITIVE_INFINITY)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(f64::NEGATIVE_INFINITY)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(0.0)), "Some(0)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(-0.0)), "Some(0)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(123.0)), "Some(123)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(-123.0)), "Some(-123)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(1.0e9)), "Some(1000000000)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(4294967295.0)),
        ///         "Some(4294967295)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(4294967296.0)),
        ///         "Some(4294967296)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(1.0e100)),
        ///         "Some(1000000000000000015902891109759918046836080856394528138978132755774783877\
        ///         2170381060813469985856815104)");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(123.1)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(123.9)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(123.5)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(124.5)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(-0.499)), "None");
        ///     assert_eq!(format!("{:?}", Integer::checked_from(-0.5)), "None");
        /// }
        /// ```
        impl CheckedFrom<$f> for Integer {
            fn checked_from(value: $f) -> Option<Integer> {
                Natural::checked_from(value.abs()).map(|n| Integer {
                    sign: value >= 0.0,
                    abs: n,
                })
            }
        }
    };
}

float_impls!(f32);
float_impls!(f64);
