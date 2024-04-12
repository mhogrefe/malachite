// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::stats::common_values_map::common_values_map_debug;
use crate::test_util::stats::median;
use itertools::Itertools;
use std::fmt::Debug;
use std::hash::Hash;

pub fn random_vecs_helper_helper<T, I: Clone + Iterator<Item = Vec<T>>>(
    xss: I,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) where
    T: Clone + Debug + Eq + Hash + Ord,
{
    let values = xss.clone().take(20).collect_vec();
    let values = values.iter().map(Vec::as_slice).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xss.clone());
    let common_values = common_values
        .iter()
        .map(|(xs, f)| (xs.as_slice(), *f))
        .collect_vec();
    let (a, ob) = median(xss.take(1000000));
    let median = (a.as_slice(), ob.as_deref());
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}
