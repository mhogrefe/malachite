use std::ops::Rem;

use comparison::traits::Min;
use num::arithmetic::traits::DivisibleBy;
use num::basic::traits::{NegativeOne, Zero};

fn _divisible_by_unsigned<T: Copy + Eq + Rem<T, Output = T> + Zero>(x: T, other: T) -> bool {
    x == T::ZERO || other != T::ZERO && x % other == T::ZERO
}

macro_rules! impl_divisible_by_unsigned {
    ($t:ident) => {
        impl DivisibleBy<$t> for $t {
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
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                _divisible_by_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_unsigned);

fn _divisible_by_signed<T: Copy + Eq + Min + NegativeOne + Rem<T, Output = T> + Zero>(
    x: T,
    other: T,
) -> bool {
    x == T::ZERO
        || x == T::MIN && other == T::NEGATIVE_ONE
        || other != T::ZERO && x % other == T::ZERO
}

macro_rules! impl_divisible_by_signed {
    ($t:ident) => {
        impl DivisibleBy<$t> for $t {
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
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                _divisible_by_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_divisible_by_signed);
