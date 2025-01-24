// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::to_hex_string;
use crate::Float;
use itertools::Itertools;

pub fn exhaustive_floats_helper_helper_with_limit<I: Clone + Iterator<Item = Float>>(
    limit: usize,
    xs: I,
    out: &[&str],
    out_hex: &[&str],
) {
    let xs_hex = xs.clone();
    assert_eq!(
        xs_hex
            .take(limit)
            .map(|f| {
                assert!(f.is_valid());
                to_hex_string(&f)
            })
            .collect_vec()
            .as_slice(),
        out_hex
    );
    assert_eq!(
        xs.take(limit)
            .map(|f| { f.to_string() })
            .collect_vec()
            .as_slice(),
        out
    );
}

pub fn exhaustive_floats_helper_helper<I: Clone + Iterator<Item = Float>>(
    xs: I,
    out: &[&str],
    out_hex: &[&str],
) {
    exhaustive_floats_helper_helper_with_limit(50, xs, out, out_hex);
}
