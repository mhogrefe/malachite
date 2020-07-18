use num::arithmetic::traits::PowerOfTwo;
use num::basic::integers::PrimitiveInteger;

#[inline]
fn _power_of_two_unsigned<T: PrimitiveInteger>(pow: u64) -> T {
    assert!(pow < T::WIDTH);
    T::ONE << pow
}

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
                _power_of_two_unsigned(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_power_of_two_unsigned);

#[inline]
pub fn _power_of_two_signed<T: PrimitiveInteger>(pow: u64) -> T {
    assert!(pow < T::WIDTH - 1);
    T::ONE << pow
}

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
                _power_of_two_signed(pow)
            }
        }
    };
}
apply_to_signeds!(impl_power_of_two_signed);
