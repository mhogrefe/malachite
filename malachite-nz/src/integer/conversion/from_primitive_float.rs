use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerFromPrimitiveFloatError;

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

        impl TryFrom<$f> for Integer {
            type Error = IntegerFromPrimitiveFloatError;

            /// Converts a primitive float to an [`Integer`].
            ///
            /// If the input isn't exactly equal to some [`Integer`], an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#try_from).
            fn try_from(value: $f) -> Result<Integer, Self::Error> {
                Natural::try_from(value.abs())
                    .map(|n| Integer {
                        sign: value >= 0.0,
                        abs: n,
                    })
                    .map_err(|_| IntegerFromPrimitiveFloatError)
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
