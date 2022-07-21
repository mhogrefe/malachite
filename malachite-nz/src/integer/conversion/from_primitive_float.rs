use crate::integer::Integer;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use crate::natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<$f> for Integer {
            /// Converts a primitive float to an [`Integer`], using the specified rounding mode.
            ///
            /// The floating-point value cannot be NaN or infinite.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite or if the rounding mode is `Exact` and `value`
            /// is not an integer.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#rounding_from).
            fn rounding_from(value: $f, rm: RoundingMode) -> Self {
                if value >= 0.0 {
                    Integer {
                        sign: true,
                        abs: Natural::rounding_from(value, rm),
                    }
                } else {
                    -Natural::rounding_from(-value, -rm)
                }
            }
        }

        impl From<$f> for Integer {
            /// Converts a primitive float to the nearest [`Integer`].
            ///
            /// Floating-point values exactly between two [`Integer`]s are rounded to the even one.
            /// The floating point value cannot be NaN or infinite.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#from).
            fn from(value: $f) -> Integer {
                let abs = Natural::from(value.abs());
                Integer {
                    sign: value >= 0.0 || abs == 0,
                    abs,
                }
            }
        }

        impl CheckedFrom<$f> for Integer {
            /// Converts a primitive float to an [`Integer`].
            ///
            /// If the input isn't exactly equal to some [`Integer`], `None` is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#checked_from).
            fn checked_from(value: $f) -> Option<Integer> {
                Natural::checked_from(value.abs()).map(|n| Integer {
                    sign: value >= 0.0,
                    abs: n,
                })
            }
        }

        impl ConvertibleFrom<$f> for Integer {
            /// Determines whether a primitive float can be exactly converted to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            fn convertible_from(value: $f) -> bool {
                Natural::convertible_from(value.abs())
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
