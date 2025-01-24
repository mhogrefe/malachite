// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::nonzero_values;

#[test]
pub fn test_nonzero_values() {
    let test = |xs: &[u32], out: &[u32]| {
        assert_eq!(nonzero_values(xs.iter().copied()).collect_vec(), out);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[1, 2, 3]);
    test(&[1, 0, 3], &[1, 3]);
    test(&[1, 2, 0, 0, 0], &[1, 2]);
    test(&[0, 0, 0], &[]);
}
