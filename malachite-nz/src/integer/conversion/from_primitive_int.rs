use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use natural::Natural;

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts a value to an `Integer`, where the value is of a primitive unsigned integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(Integer::from(123u32).to_string(), "123");
            /// ```
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
            /// Converts a value to an `Integer`, where the value is of a primitive signed integer
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!(Integer::from(-123i32).to_string(), "-123");
            /// ```
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
