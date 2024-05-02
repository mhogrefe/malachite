// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::floats::PrimitiveFloat;
use crate::num::float::NiceFloat;
use itertools::Itertools;

pub fn exhaustive_primitive_floats_helper_helper_with_limit<
    T: PrimitiveFloat,
    I: Iterator<Item = T>,
>(
    limit: usize,
    xs: I,
    out: &[T],
) {
    assert_eq!(
        xs.take(limit).map(NiceFloat).collect_vec(),
        out.iter().copied().map(NiceFloat).collect_vec()
    );
}

pub fn exhaustive_primitive_floats_helper_helper<T: PrimitiveFloat, I: Iterator<Item = T>>(
    xs: I,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper_with_limit(50, xs, out);
}

pub fn exhaustive_primitive_floats_helper_helper_with_reverse<
    T: PrimitiveFloat,
    I: Clone + DoubleEndedIterator<Item = T>,
>(
    xs: I,
    first_20: &[T],
    last_20: &[T],
) {
    assert_eq!(
        xs.clone().take(20).map(NiceFloat).collect_vec(),
        first_20.iter().copied().map(NiceFloat).collect_vec()
    );
    let mut reversed = xs.rev().take(20).map(NiceFloat).collect_vec();
    reversed.reverse();
    assert_eq!(
        reversed,
        last_20.iter().copied().map(NiceFloat).collect_vec()
    );
}
