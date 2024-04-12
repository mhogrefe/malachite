// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{BitScan, TrailingZeros};

fn index_of_next_false_bit_unsigned<T: PrimitiveUnsigned>(x: T, start: u64) -> u64 {
    if start >= T::WIDTH {
        start
    } else {
        TrailingZeros::trailing_zeros(!(x | T::low_mask(start)))
    }
}

fn index_of_next_true_bit_unsigned<T: PrimitiveUnsigned>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH {
        None
    } else {
        let index = TrailingZeros::trailing_zeros(x & !T::low_mask(start));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

macro_rules! impl_bit_scan_unsigned {
    ($t:ident) => {
        impl BitScan for $t {
            /// Given a number and a starting index, searches the number for the smallest index of a
            /// `false` bit that is greater than or equal to the starting index.
            ///
            /// Since the number is unsigned and therefore has an implicit prefix of infinitely-many
            /// zeros, this function always returns a value.
            ///
            /// Starting beyond the type's width is allowed; the result is the starting index.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_scan#index_of_next_false_bit).
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                Some(index_of_next_false_bit_unsigned(self, start))
            }

            /// Given a number and a starting index, searches the number for the smallest index of a
            /// `true` bit that is greater than or equal to the starting index.
            ///
            /// If the starting index is greater than or equal to the type's width, the result is
            /// `None` since there are no `true` bits past that point.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_scan#index_of_next_true_bit).
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                index_of_next_true_bit_unsigned(self, start)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_scan_unsigned);

fn index_of_next_false_bit_signed<T: PrimitiveSigned>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH - 1 {
        if x >= T::ZERO {
            Some(start)
        } else {
            None
        }
    } else {
        let index = TrailingZeros::trailing_zeros(!(x | T::low_mask(start)));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

fn index_of_next_true_bit_signed<T: PrimitiveSigned>(x: T, start: u64) -> Option<u64> {
    if start >= T::WIDTH - 1 {
        if x >= T::ZERO {
            None
        } else {
            Some(start)
        }
    } else {
        let index = TrailingZeros::trailing_zeros(x & !T::low_mask(start));
        if index == T::WIDTH {
            None
        } else {
            Some(index)
        }
    }
}

macro_rules! impl_bit_scan_signed {
    ($t:ident) => {
        impl BitScan for $t {
            /// Given a number and a starting index, searches the number for the smallest index of a
            /// `false` bit that is greater than or equal to the starting index.
            ///
            /// If the starting index is greater than or equal to the type's width, then the result
            /// depends on whether the number is negative. If it is, then the result is `None` since
            /// there are no `false` bits past that point. If the number is non-negative, then the
            /// result is the starting index.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_scan#index_of_next_false_bit).
            #[inline]
            fn index_of_next_false_bit(self, start: u64) -> Option<u64> {
                index_of_next_false_bit_signed(self, start)
            }

            /// Given a number and a starting index, searches the number for the smallest index of a
            /// `true` bit that is greater than or equal to the starting index.
            ///
            /// If the starting index is greater than or equal to the type's width, then the result
            /// depends on whether the number is non-negative. If it is, then the result is `None`
            /// since there are no `true` bits past that point. If the number is negative, then the
            /// result is the starting index.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_scan#index_of_next_true_bit).
            #[inline]
            fn index_of_next_true_bit(self, start: u64) -> Option<u64> {
                index_of_next_true_bit_signed(self, start)
            }
        }
    };
}
apply_to_signeds!(impl_bit_scan_signed);
