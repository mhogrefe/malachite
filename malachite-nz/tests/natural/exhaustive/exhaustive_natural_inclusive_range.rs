// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
use malachite_nz::natural::Natural;

fn expected_range_len(a: &Natural, b: &Natural) -> usize {
    usize::exact_from(b) - usize::exact_from(a) + 1
}

fn exhaustive_natural_inclusive_range_helper(a: Natural, b: Natural, values: &str) {
    let xs = exhaustive_natural_inclusive_range(a.clone(), b.clone())
        .take(20)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
    let len = expected_range_len(&a, &b);
    assert_eq!(
        exhaustive_natural_inclusive_range(a.clone(), b.clone()).count(),
        len
    );
    let mut init = exhaustive_natural_inclusive_range(a, b)
        .rev()
        .skip(len.saturating_sub(20))
        .collect_vec();
    init.reverse();
    assert_eq!(xs, init.to_debug_string());
}

#[test]
fn test_exhaustive_natural_inclusive_range() {
    exhaustive_natural_inclusive_range_helper(Natural::ZERO, Natural::ZERO, "[0]");
    exhaustive_natural_inclusive_range_helper(Natural::ZERO, Natural::ONE, "[0, 1]");
    exhaustive_natural_inclusive_range_helper(
        Natural::ZERO,
        Natural::exact_from(10),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]",
    );
    exhaustive_natural_inclusive_range_helper(
        Natural::exact_from(10),
        Natural::exact_from(20),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]",
    );
    exhaustive_natural_inclusive_range_helper(
        Natural::exact_from(10),
        Natural::exact_from(100),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]",
    );
}

#[test]
#[should_panic]
fn exhaustive_natural_inclusive_range_fail() {
    exhaustive_natural_inclusive_range(Natural::ONE, Natural::ZERO);
}
