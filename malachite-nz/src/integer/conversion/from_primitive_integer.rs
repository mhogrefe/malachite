use malachite_base::num::arithmetic::traits::UnsignedAbs;

use integer::Integer;
use natural::Natural;

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts a value to a `Integer`, where the value is of a primitive unsigned integer
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
            fn from(u: $t) -> Integer {
                Integer {
                    sign: true,
                    abs: Natural::from(u),
                }
            }
        }
    };
}

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Integer {
            /// Converts a value to a `Integer`, where the value is of a primitive signed integer
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
            fn from(i: $t) -> Integer {
                Integer {
                    sign: i >= 0,
                    abs: Natural::from(i.unsigned_abs()),
                }
            }
        }
    };
}

impl_from_unsigned!(u8);
impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(u128);
impl_from_unsigned!(usize);
impl_from_signed!(i8);
impl_from_signed!(i16);
impl_from_signed!(i32);
impl_from_signed!(i64);
impl_from_signed!(i128);
impl_from_signed!(isize);
