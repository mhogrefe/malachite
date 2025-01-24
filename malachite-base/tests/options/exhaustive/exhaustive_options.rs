// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::nevers::Never;
use malachite_base::options::exhaustive::exhaustive_options;
use std::fmt::Debug;

fn exhaustive_options_helper<T: Clone + Debug + Eq>(xs: &[T], out: &[Option<T>]) {
    assert_eq!(
        exhaustive_options(xs.iter().cloned())
            .collect_vec()
            .as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_options() {
    exhaustive_options_helper::<Never>(&[], &[None]);
    exhaustive_options_helper(&[5], &[None, Some(5)]);
    exhaustive_options_helper(&[1, 2, 3], &[None, Some(1), Some(2), Some(3)]);
    exhaustive_options_helper(
        &[Some(2), None, Some(5)],
        &[None, Some(Some(2)), Some(None), Some(Some(5))],
    );
}
