// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_unsigneds;

fn exhaustive_unsigneds_helper<T: PrimitiveUnsigned>()
where
    u8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_unsigneds::<T>()
            .map(u8::exact_from)
            .take(20)
            .collect_vec(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    );
}

fn exhaustive_unsigneds_long_helper<T: PrimitiveUnsigned>(last_20: &[T]) {
    let expected_len = usize::power_of_2(T::WIDTH);
    let xs = exhaustive_unsigneds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect_vec(), last_20);
}

#[test]
fn test_exhaustive_unsigneds() {
    apply_fn_to_unsigneds!(exhaustive_unsigneds_helper);

    exhaustive_unsigneds_long_helper::<u8>(&[
        236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
        254, 255,
    ]);
    exhaustive_unsigneds_long_helper::<u16>(&[
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
}
