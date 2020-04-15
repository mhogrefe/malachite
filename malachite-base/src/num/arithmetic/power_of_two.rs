use num::arithmetic::traits::PowerOfTwo;
use num::basic::integers::PrimitiveInteger;

macro_rules! impl_arithmetic_traits {
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

impl_arithmetic_traits!(u8);
impl_arithmetic_traits!(u16);
impl_arithmetic_traits!(u32);
impl_arithmetic_traits!(u64);
impl_arithmetic_traits!(u128);
impl_arithmetic_traits!(usize);
