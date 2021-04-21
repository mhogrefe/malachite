use num::arithmetic::traits::PowerOf2;
use num::basic::integers::PrimitiveInt;

fn _power_of_2_unsigned<T: PrimitiveInt>(pow: u64) -> T {
    assert!(pow < T::WIDTH);
    T::ONE << pow
}

macro_rules! impl_power_of_2_unsigned {
    ($t:ident) => {
        impl PowerOf2 for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOf2;
            ///
            /// assert_eq!(u16::power_of_2(0), 1);
            /// assert_eq!(u8::power_of_2(3), 8);
            /// assert_eq!(u64::power_of_2(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                _power_of_2_unsigned(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_power_of_2_unsigned);

fn _power_of_2_signed<T: PrimitiveInt>(pow: u64) -> T {
    assert!(pow < T::WIDTH - 1);
    T::ONE << pow
}

macro_rules! impl_power_of_2_signed {
    ($t:ident) => {
        impl PowerOf2 for $t {
            /// Computes 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `pow` is greater than or equal to the width of `$t` minus 1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::PowerOf2;
            ///
            /// assert_eq!(i16::power_of_2(0), 1);
            /// assert_eq!(i8::power_of_2(3), 8);
            /// assert_eq!(i64::power_of_2(40), 1 << 40);
            /// ```
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                _power_of_2_signed(pow)
            }
        }
    };
}
apply_to_signeds!(impl_power_of_2_signed);
