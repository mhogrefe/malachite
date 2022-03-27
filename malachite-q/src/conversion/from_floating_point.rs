use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use Rational;

macro_rules! float_impls {
    ($f: ident) => {
        impl From<$f> for Rational {
            /// Converts an `f32` or `f64` to the equivalent `Rational`. The floating point value
            /// cannot be NaN or infinite.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ the absolute value of the
            /// float's scientific exponent.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite.
            ///
            /// # Examples
            /// See the documentation of the `conversion::from_floating_point` module.
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
float_impls!(f32);
float_impls!(f64);
