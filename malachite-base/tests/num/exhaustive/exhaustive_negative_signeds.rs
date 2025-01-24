// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::num::exhaustive::{exhaustive_natural_signeds, exhaustive_negative_signeds};

fn exhaustive_negative_signeds_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
{
    let xs = exhaustive_negative_signeds::<T>()
        .map(i8::exact_from)
        .take(20)
        .collect_vec();
    assert_eq!(
        xs,
        &[
            -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19,
            -20
        ]
    );
    assert!(itertools::equal(
        xs,
        exhaustive_natural_signeds::<T>()
            .map(|x| !i8::exact_from(x))
            .take(20)
    ));
}

fn exhaustive_negative_signeds_long_helper<T: PrimitiveSigned>(last_20: &[T]) {
    let expected_len = usize::power_of_2(T::WIDTH - 1);
    let xs = exhaustive_negative_signeds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect_vec(), last_20);
}

#[test]
fn test_exhaustive_negative_signeds() {
    apply_fn_to_signeds!(exhaustive_negative_signeds_helper);

    exhaustive_negative_signeds_long_helper::<i8>(&[
        -109, -110, -111, -112, -113, -114, -115, -116, -117, -118, -119, -120, -121, -122, -123,
        -124, -125, -126, -127, -128,
    ]);
    exhaustive_negative_signeds_long_helper::<i16>(&[
        -32749,
        -32750,
        -32751,
        -0x7ff0,
        -0x7ff1,
        -0x7ff2,
        -0x7ff3,
        -0x7ff4,
        -0x7ff5,
        -0x7ff6,
        -0x7ff7,
        -0x7ff8,
        -0x7ff9,
        -0x7ffa,
        -0x7ffb,
        -0x7ffc,
        -0x7ffd,
        -0x7ffe,
        i16::MIN + 1,
        i16::MIN,
    ]);
}
