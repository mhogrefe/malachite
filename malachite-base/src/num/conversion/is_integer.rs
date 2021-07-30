use num::basic::integers::PrimitiveInt;
use num::conversion::traits::{IsInteger, WrappingFrom};
use num::float::PrimitiveFloat;
use num::logic::traits::TrailingZeros;

impl<T: PrimitiveInt> IsInteger for T {
    /// Determines whether a value is an integer.
    ///
    /// For primitive integer types this always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::IsInteger;
    ///
    /// assert_eq!(0.is_integer(), true);
    /// assert_eq!(1.is_integer(), true);
    /// assert_eq!(100.is_integer(), true);
    /// assert_eq!((-1).is_integer(), true);
    /// assert_eq!((-100).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        true
    }
}

fn _is_integer<T: PrimitiveFloat>(x: T) -> bool {
    if x.is_nan() || x.is_infinite() {
        false
    } else if x == T::ZERO {
        true
    } else {
        let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
        raw_exponent != 0
            && i64::wrapping_from(
                raw_exponent
                    + if raw_mantissa == 0 {
                        T::MANTISSA_WIDTH
                    } else {
                        TrailingZeros::trailing_zeros(raw_mantissa)
                    },
            ) > -T::MIN_EXPONENT
    }
}

macro_rules! impl_is_integer_primitive_float {
    ($t:ident) => {
        impl IsInteger for $t {
            /// Determines whether a value is an integer.
            ///
            /// $f(x) = (x \in \Z)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::is_integer` module.
            #[inline]
            fn is_integer(self) -> bool {
                _is_integer(self)
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_integer_primitive_float);
