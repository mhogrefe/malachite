use crate::Rational;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;

macro_rules! float_impls {
    ($f: ident) => {
        impl From<$f> for Rational {
            /// Converts a primitive float to the equivalent [`Rational`]. The floating point value
            /// cannot be `NaN` or infinite.
            ///
            /// This conversion is literal. For example, `Rational::from(0.1f32)` evaluates to
            /// $13421773/134217728$. If you want $1/10$ instead, use
            /// [`from_float_simplest`](Rational::from_float_simplest); that function returns the
            /// simplest [`Rational`] that rounds to the specified float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `value.sci_exponent().abs()`.
            ///
            /// # Panics
            /// Panics if `value` is `NaN` or infinite.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#from).
            fn from(value: $f) -> Rational {
                if value == 0.0 {
                    Rational::ZERO
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    let x = Rational::from(mantissa) << exponent;
                    if value > 0.0 {
                        x
                    } else {
                        -x
                    }
                }
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
