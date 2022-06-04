use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use Rational;

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Rational {
            /// Converts an unsigned primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
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
            /// Converts a signed primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
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
