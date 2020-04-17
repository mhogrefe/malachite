use num::arithmetic::traits::PowerOfTwo;
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_power_of_two_unsigned {
    ($t:ident) => {
        impl PowerOfTwo for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t`.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOfTwo;
            ///
            /// assert_eq!(u16::power_of_two(0), 1);
            /// assert_eq!(u8::power_of_two(3), 8);
            /// assert_eq!(u64::power_of_two(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_two(pow: u64) -> $t {
                assert!(pow < $t::WIDTH);
                1 << pow
            }
        }
    };
}

impl_power_of_two_unsigned!(u8);
impl_power_of_two_unsigned!(u16);
impl_power_of_two_unsigned!(u32);
impl_power_of_two_unsigned!(u64);
impl_power_of_two_unsigned!(u128);
impl_power_of_two_unsigned!(usize);

macro_rules! impl_power_of_two_signed {
    ($t:ident) => {
        impl PowerOfTwo for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t` minus 1.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOfTwo;
            ///
            /// assert_eq!(i16::power_of_two(0), 1);
            /// assert_eq!(i8::power_of_two(3), 8);
            /// assert_eq!(i64::power_of_two(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_two(pow: u64) -> $t {
                assert!(pow < $t::WIDTH - 1);
                1 << pow
            }
        }
    };
}

impl_power_of_two_signed!(i8);
impl_power_of_two_signed!(i16);
impl_power_of_two_signed!(i32);
impl_power_of_two_signed!(i64);
impl_power_of_two_signed!(i128);
impl_power_of_two_signed!(isize);
