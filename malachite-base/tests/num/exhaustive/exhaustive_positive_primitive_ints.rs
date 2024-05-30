// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;

fn exhaustive_positive_primitive_ints_helper<T: PrimitiveInt>()
where
    u8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_positive_primitive_ints::<T>()
            .map(u8::exact_from)
            .take(20)
            .collect_vec(),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    );
}

fn exhaustive_positive_primitive_ints_long_helper<T: PrimitiveInt>(last_20: &[T]) {
    let expected_len = if T::MIN == T::ZERO {
        usize::power_of_2(T::WIDTH) - 1
    } else {
        usize::power_of_2(T::WIDTH - 1) - 1
    };
    let xs = exhaustive_positive_primitive_ints::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect_vec(), last_20);
}

#[test]
fn test_exhaustive_positive_primitive_ints() {
    apply_fn_to_primitive_ints!(exhaustive_positive_primitive_ints_helper);

    exhaustive_positive_primitive_ints_long_helper::<u8>(&[
        236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
        254, 255,
    ]);
    exhaustive_positive_primitive_ints_long_helper::<i8>(&[
        108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
        126, 127,
    ]);
    exhaustive_positive_primitive_ints_long_helper::<u16>(&[
        65516,
        65517,
        65518,
        65519,
        0xfff0,
        0xfff1,
        0xfff2,
        0xfff3,
        0xfff4,
        0xfff5,
        0xfff6,
        0xfff7,
        0xfff8,
        0xfff9,
        0xfffa,
        0xfffb,
        0xfffc,
        0xfffd,
        u16::MAX - 1,
        u16::MAX,
    ]);
    exhaustive_positive_primitive_ints_long_helper::<i16>(&[
        32748,
        32749,
        32750,
        32751,
        0x7ff0,
        0x7ff1,
        0x7ff2,
        0x7ff3,
        0x7ff4,
        0x7ff5,
        0x7ff6,
        0x7ff7,
        0x7ff8,
        0x7ff9,
        0x7ffa,
        0x7ffb,
        0x7ffc,
        0x7ffd,
        i16::MAX - 1,
        i16::MAX,
    ]);
}
