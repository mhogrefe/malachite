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
use malachite_base::num::exhaustive::primitive_int_increasing_range;
use std::panic::catch_unwind;

fn expected_range_len<T: PrimitiveInt>(a: T, b: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(b).wrapping_sub(usize::wrapping_from(a))
}

fn primitive_int_increasing_range_helper_helper<T: PrimitiveInt>(a: T, b: T, values: &[i8])
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = primitive_int_increasing_range::<T>(a, b)
        .map(i8::exact_from)
        .take(20)
        .collect_vec();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a, b);
        assert_eq!(primitive_int_increasing_range(a, b).count(), len);
        let mut init = primitive_int_increasing_range::<T>(a, b)
            .rev()
            .skip(len.saturating_sub(20))
            .map(i8::exact_from)
            .collect_vec();
        init.reverse();
        assert_eq!(xs, init);
    }
}

fn primitive_int_increasing_range_rev_helper<T: PrimitiveInt>(a: T, b: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let xs = primitive_int_increasing_range::<T>(a, b)
        .rev()
        .take(20)
        .collect_vec();
    assert_eq!(xs, rev_values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a, b);
        assert_eq!(primitive_int_increasing_range(a, b).rev().count(), len);
        let mut tail = primitive_int_increasing_range::<T>(a, b)
            .skip(len.saturating_sub(20))
            .collect_vec();
        tail.reverse();
        assert_eq!(xs, tail);
    }
}

fn primitive_int_increasing_range_helper<T: PrimitiveInt>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    primitive_int_increasing_range_helper_helper(T::exact_from(5), T::exact_from(5), &[]);
    primitive_int_increasing_range_helper_helper(T::exact_from(5), T::exact_from(6), &[5]);
    primitive_int_increasing_range_helper_helper(T::ONE, T::exact_from(7), &[1, 2, 3, 4, 5, 6]);
    primitive_int_increasing_range_helper_helper(
        T::exact_from(10),
        T::exact_from(20),
        &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    primitive_int_increasing_range_helper_helper(
        T::ZERO,
        T::MAX,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    primitive_int_increasing_range_helper_helper(
        T::ZERO,
        T::MAX - T::ONE,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
}

fn primitive_int_increasing_range_helper_signed<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    primitive_int_increasing_range_helper_helper(
        T::exact_from(-20),
        T::exact_from(-10),
        &[-20, -19, -18, -17, -16, -15, -14, -13, -12, -11],
    );
    primitive_int_increasing_range_helper_helper(
        T::exact_from(-100),
        T::exact_from(100),
        &[
            -100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84,
            -83, -82, -81,
        ],
    );
}

#[test]
fn test_primitive_int_increasing_range() {
    apply_fn_to_primitive_ints!(primitive_int_increasing_range_helper);
    apply_fn_to_signeds!(primitive_int_increasing_range_helper_signed);

    primitive_int_increasing_range_rev_helper::<u8>(
        0,
        u8::MAX,
        &[
            254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239, 238,
            237, 236, 235,
        ],
    );
    primitive_int_increasing_range_rev_helper::<u8>(
        0,
        u8::MAX - 1,
        &[
            253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239, 238, 237,
            236, 235, 234,
        ],
    );
    primitive_int_increasing_range_rev_helper::<u16>(
        0,
        u16::MAX,
        &[
            65534, 65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524, 65523,
            65522, 65521, 65520, 65519, 65518, 65517, 65516, 65515,
        ],
    );
    primitive_int_increasing_range_rev_helper::<u16>(
        0,
        u16::MAX - 1,
        &[
            65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524, 65523, 65522,
            65521, 65520, 65519, 65518, 65517, 65516, 65515, 65514,
        ],
    );

    primitive_int_increasing_range_helper_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            -128, -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115,
            -114, -113, -112, -111, -110, -109,
        ],
    );
    primitive_int_increasing_range_helper_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115, -114,
            -113, -112, -111, -110, -109, -108,
        ],
    );

    primitive_int_increasing_range_rev_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110,
            109, 108, 107,
        ],
    );
    primitive_int_increasing_range_rev_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110, 109,
            108, 107, 106,
        ],
    );
    primitive_int_increasing_range_rev_helper::<i16>(
        i16::MIN,
        i16::MAX,
        &[
            32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756, 32755,
            32754, 32753, 32752, 32751, 32750, 32749, 32748, 32747,
        ],
    );
    primitive_int_increasing_range_rev_helper::<i16>(
        i16::MIN + 1,
        i16::MAX - 1,
        &[
            32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756, 32755, 32754,
            32753, 32752, 32751, 32750, 32749, 32748, 32747, 32746,
        ],
    );
}

fn primitive_int_increasing_range_fail_helper<T: PrimitiveInt>() {
    assert_panic!(primitive_int_increasing_range::<T>(T::ONE, T::ZERO));
}

#[test]
fn primitive_int_increasing_range_fail() {
    apply_fn_to_primitive_ints!(primitive_int_increasing_range_fail_helper);
}
