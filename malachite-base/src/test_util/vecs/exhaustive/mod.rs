// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use std::fmt::Debug;

pub fn exhaustive_vecs_helper_helper<T, I: Iterator<Item = Vec<T>>>(xss: I, out: &[&[T]])
where
    T: Clone + Debug + Eq,
{
    let xss = xss.take(20).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

pub fn exhaustive_vecs_small_helper_helper<T, I: Clone + Iterator<Item = Vec<T>>>(
    xss: I,
    out_len: usize,
    out: &[&[T]],
) where
    T: Clone + Debug + Eq,
{
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(
        xss_prefix
            .iter()
            .map(Vec::as_slice)
            .collect_vec()
            .as_slice(),
        out
    );
    assert_eq!(xss.count(), out_len);
}
