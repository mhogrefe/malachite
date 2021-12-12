use comparison::traits::Min;
use num::arithmetic::traits::DivisibleBy;
use num::basic::traits::{NegativeOne, Zero};
use std::ops::Rem;

fn divisible_by_unsigned<T: Copy + Eq + Rem<T, Output = T> + Zero>(x: T, other: T) -> bool {
    x == T::ZERO || other != T::ZERO && x % other == T::ZERO
}

macro_rules! impl_divisible_by_unsigned {
    ($t:ident) => {
        impl DivisibleBy<$t> for $t {
            /// Returns whether a value is divisible by another value; in other words, whether the
            /// first value is a multiple of the second.
            ///
            /// This means that zero is divisible by any number, including zero; but a nonzero
            /// number is never divisible by zero.
            ///
            /// $f(x, m) = (m|x)$.
            ///
            /// $f(x, m) = (\exists k \in \N \ x = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::divisible_by` module.
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                divisible_by_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_unsigned);

fn divisible_by_signed<T: Copy + Eq + Min + NegativeOne + Rem<T, Output = T> + Zero>(
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
            /// first value is a multiple of the second.
            ///
            /// This means that zero is divisible by any number, including zero; but a nonzero
            /// number is never divisible by zero.
            ///
            /// $f(x, m) = (m|x)$.
            ///
            /// $f(x, m) = (\exists k \in \Z \ x = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::divisible_by` module.
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                divisible_by_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_divisible_by_signed);
