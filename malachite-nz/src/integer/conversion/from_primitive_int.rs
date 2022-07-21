use crate::integer::Integer;
use crate::natural::Natural;

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts an unsigned primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Integer {
                Integer {
                    sign: true,
                    abs: Natural::from(u),
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts a signed primitive integer to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Integer {
                Integer {
                    sign: i >= 0,
                    abs: Natural::from(i.unsigned_abs()),
                }
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
