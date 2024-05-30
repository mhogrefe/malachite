// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_signeds;

fn exhaustive_signeds_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_signeds::<T>()
            .map(i8::exact_from)
            .take(20)
            .collect_vec(),
        &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10]
    );
}

fn exhaustive_signeds_long_helper<T: PrimitiveSigned>(last_20: &[T]) {
    let expected_len = usize::power_of_2(T::WIDTH);
    let xs = exhaustive_signeds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect_vec(), last_20);
}

#[test]
fn test_exhaustive_signeds() {
    apply_fn_to_signeds!(exhaustive_signeds_helper);

    exhaustive_signeds_long_helper::<i8>(&[
        -118, 119, -119, 120, -120, 121, -121, 122, -122, 123, -123, 124, -124, 125, -125, 126,
        -126, 127, -127, -128,
    ]);
    exhaustive_signeds_long_helper::<i16>(&[
        -0x7ff6,
        0x7ff7,
        -0x7ff7,
        0x7ff8,
        -0x7ff8,
        0x7ff9,
        -0x7ff9,
        0x7ffa,
        -0x7ffa,
        0x7ffb,
        -0x7ffb,
        0x7ffc,
        -0x7ffc,
        0x7ffd,
        -0x7ffd,
        i16::MAX - 1,
        -0x7ffe,
        i16::MAX,
        i16::MIN + 1,
        i16::MIN,
    ]);
}
