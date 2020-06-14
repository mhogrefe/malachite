use num::arithmetic::traits::DivisibleBy;

macro_rules! impl_divisible_by_unsigned {
    ($t:ident) => {
        impl DivisibleBy for $t {
            /// Returns whether a value is divisible by another value; in other words, whether the
            /// first value is a multiple of the second. This means that zero is divisible by any
            /// number, including zero; but a nonzero number is never divisible by zero.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivisibleBy;
            ///
            /// assert_eq!(0u8.divisible_by(0), true);
            /// assert_eq!(100u16.divisible_by(3), false);
            /// assert_eq!(102u32.divisible_by(3), true);
            /// ```
            fn divisible_by(self, other: $t) -> bool {
                self == 0 || other != 0 && self % other == 0
            }
        }
    };
}
impl_divisible_by_unsigned!(u8);
impl_divisible_by_unsigned!(u16);
impl_divisible_by_unsigned!(u32);
impl_divisible_by_unsigned!(u64);
impl_divisible_by_unsigned!(u128);
impl_divisible_by_unsigned!(usize);

macro_rules! impl_divisible_by_signed {
    ($t:ident) => {
        impl DivisibleBy for $t {
            /// Returns whether a value is divisible by another value; in other words, whether the
            /// first value is a multiple of the second. This means that zero is divisible by any
            /// number, including zero; but a nonzero number is never divisible by zero.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivisibleBy;
            ///
            /// assert_eq!(0i8.divisible_by(0), true);
            /// assert_eq!((-100i16).divisible_by(-3), false);
            /// assert_eq!(102i32.divisible_by(-3), true);
            /// ```
            fn divisible_by(self, other: $t) -> bool {
                self == 0 || self == $t::MIN && other == -1 || other != 0 && self % other == 0
            }
        }
    };
}
impl_divisible_by_signed!(i8);
impl_divisible_by_signed!(i16);
impl_divisible_by_signed!(i32);
impl_divisible_by_signed!(i64);
impl_divisible_by_signed!(i128);
impl_divisible_by_signed!(isize);
