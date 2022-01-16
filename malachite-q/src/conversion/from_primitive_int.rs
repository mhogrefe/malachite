use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use Rational;

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Rational {
            /// Converts a value to an `Integer`, where the value is of a primitive unsigned integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `conversion::from_primitive_int` module.
            #[inline]
            fn from(u: $t) -> Rational {
                Rational {
                    sign: true,
                    numerator: Natural::from(u),
                    denominator: Natural::ONE,
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Rational {
            /// Converts a value to an `Integer`, where the value is of a primitive signed integer
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `conversion::from_primitive_int` module.
            #[inline]
            fn from(i: $t) -> Rational {
                Rational {
                    sign: i >= 0,
                    numerator: Natural::from(i.unsigned_abs()),
                    denominator: Natural::ONE,
                }
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
