// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};

fn hamming_distance_unsigned<T: PrimitiveUnsigned>(x: T, y: T) -> u64 {
    CountOnes::count_ones(x ^ y)
}

macro_rules! impl_hamming_distance_unsigned {
    ($t:ident) => {
        impl HammingDistance<$t> for $t {
            /// Returns the Hamming distance between two numbers, or the number of bit flips needed
            /// to turn one into the other.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::hamming_distance#hamming_distance).
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                hamming_distance_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_hamming_distance_unsigned);

fn checked_hamming_distance_signed<T: PrimitiveSigned>(x: T, y: T) -> Option<u64> {
    if (x >= T::ZERO) == (y >= T::ZERO) {
        Some(CountOnes::count_ones(x ^ y))
    } else {
        None
    }
}

macro_rules! impl_checked_hamming_distance_signed {
    ($t:ident) => {
        impl CheckedHammingDistance<$t> for $t {
            /// Returns the Hamming distance between two numbers, or the number of bit flips needed
            /// to turn one into the other.
            ///
            /// If the two numbers have opposite signs, then the number of flips would be infinite,
            /// so the result is `None`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::hamming_distance#checked_hamming_distance).
            #[inline]
            fn checked_hamming_distance(self, other: $t) -> Option<u64> {
                checked_hamming_distance_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_checked_hamming_distance_signed);
