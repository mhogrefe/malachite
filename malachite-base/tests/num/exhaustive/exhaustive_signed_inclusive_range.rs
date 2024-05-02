// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::exhaustive_signed_inclusive_range;
use std::panic::catch_unwind;

fn expected_range_len<T: PrimitiveSigned>(a: T, b: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(b).wrapping_sub(usize::wrapping_from(a)) + 1
}

fn exhaustive_signed_inclusive_range_helper_helper<T: PrimitiveSigned>(a: T, b: T, values: &[i8])
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = exhaustive_signed_inclusive_range::<T>(a, b)
        .map(i8::exact_from)
        .take(20)
        .collect_vec();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        assert_eq!(
            exhaustive_signed_inclusive_range(a, b).count(),
            expected_range_len(a, b)
        );
    }
}

fn exhaustive_signed_inclusive_range_rev_helper<T: PrimitiveSigned>(a: T, b: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let len = expected_range_len(a, b);
    assert_eq!(exhaustive_signed_inclusive_range(a, b).count(), len);
    let mut tail = exhaustive_signed_inclusive_range::<T>(a, b)
        .skip(len.saturating_sub(20))
        .collect_vec();
    tail.reverse();
    assert_eq!(tail, rev_values);
}

fn exhaustive_signed_inclusive_range_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    exhaustive_signed_inclusive_range_helper_helper(T::ZERO, T::ZERO, &[0]);
    exhaustive_signed_inclusive_range_helper_helper(
        T::ZERO,
        T::exact_from(9),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::exact_from(10),
        T::exact_from(19),
        &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::ZERO,
        T::MAX,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::ZERO,
        T::MAX - T::ONE,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::exact_from(-20),
        T::exact_from(-11),
        &[-11, -12, -13, -14, -15, -16, -17, -18, -19, -20],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::exact_from(-100),
        T::exact_from(99),
        &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::MIN,
        T::MAX,
        &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10],
    );
    exhaustive_signed_inclusive_range_helper_helper(
        T::MIN + T::ONE,
        T::MAX - T::ONE,
        &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10],
    );
}

#[test]
fn test_exhaustive_signed_inclusive_range() {
    apply_fn_to_signeds!(exhaustive_signed_inclusive_range_helper);

    exhaustive_signed_inclusive_range_rev_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            -128, -127, 127, -126, 126, -125, 125, -124, 124, -123, 123, -122, 122, -121, 121,
            -120, 120, -119, 119, -118,
        ],
    );
    exhaustive_signed_inclusive_range_rev_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            -127, -126, 126, -125, 125, -124, 124, -123, 123, -122, 122, -121, 121, -120, 120,
            -119, 119, -118, 118, -117,
        ],
    );
    exhaustive_signed_inclusive_range_rev_helper::<i16>(
        i16::MIN,
        i16::MAX,
        &[
            -32768, -32767, 32767, -32766, 32766, -32765, 32765, -32764, 32764, -32763, 32763,
            -32762, 32762, -32761, 32761, -32760, 32760, -32759, 32759, -32758,
        ],
    );
    exhaustive_signed_inclusive_range_rev_helper::<i16>(
        i16::MIN + 1,
        i16::MAX - 1,
        &[
            -32767, -32766, 32766, -32765, 32765, -32764, 32764, -32763, 32763, -32762, 32762,
            -32761, 32761, -32760, 32760, -32759, 32759, -32758, 32758, -32757,
        ],
    );
}

fn exhaustive_signed_inclusive_range_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(exhaustive_signed_inclusive_range::<T>(T::ONE, T::ZERO));
}

#[test]
fn exhaustive_signed_inclusive_range_fail() {
    apply_fn_to_signeds!(exhaustive_signed_inclusive_range_fail_helper);
}
