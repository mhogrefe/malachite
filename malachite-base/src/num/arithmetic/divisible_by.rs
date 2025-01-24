// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::DivisibleBy;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn divisible_by_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> bool {
    x == T::ZERO || other != T::ZERO && x % other == T::ZERO
}

macro_rules! impl_divisible_by_unsigned {
    ($t:ident) => {
        impl DivisibleBy<$t> for $t {
            /// Returns whether a number is divisible by another number; in other words, whether the
            /// first number is a multiple of the second.
            ///
            /// This means that zero is divisible by any number, including zero; but a nonzero
            /// number is never divisible by zero.
            ///
            /// $f(x, m) = (m|x)$.
            ///
            /// $f(x, m) = (\exists k \in \N : x = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::divisible_by#divisible_by).
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                divisible_by_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_unsigned);

fn divisible_by_signed<T: PrimitiveSigned>(x: T, other: T) -> bool {
    x == T::ZERO
        || x == T::MIN && other == T::NEGATIVE_ONE
        || other != T::ZERO && x % other == T::ZERO
}

macro_rules! impl_divisible_by_signed {
    ($t:ident) => {
        impl DivisibleBy<$t> for $t {
            /// Returns whether a number is divisible by another number; in other words, whether the
            /// first number is a multiple of the second.
            ///
            /// This means that zero is divisible by any number, including zero; but a nonzero
            /// number is never divisible by zero.
            ///
            /// $f(x, m) = (m|x)$.
            ///
            /// $f(x, m) = (\exists k \in \Z : \ x = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::divisible_by#divisible_by).
            #[inline]
            fn divisible_by(self, other: $t) -> bool {
                divisible_by_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_divisible_by_signed);
