// Copyright © 2024 Mikhail Hogrefe
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
use malachite_nz::integer::exhaustive::exhaustive_integer_inclusive_range;
use malachite_nz::integer::Integer;

fn expected_range_len(a: &Integer, b: &Integer) -> usize {
    match (*a >= 0, *b >= 0) {
        (false, false) => usize::exact_from(&-a) - usize::exact_from(&-b) + 1,
        (false, true) => usize::exact_from(&-a) + usize::exact_from(b) + 1,
        (true, false) => panic!(),
        (true, true) => usize::exact_from(b) - usize::exact_from(a) + 1,
    }
}

fn exhaustive_integer_inclusive_range_helper(a: &Integer, b: &Integer, values: &str) {
    let xs = exhaustive_integer_inclusive_range(a.clone(), b.clone())
        .take(20)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
    assert_eq!(
        exhaustive_integer_inclusive_range(a.clone(), b.clone()).count(),
        expected_range_len(a, b)
    );
}

fn exhaustive_integer_inclusive_range_rev_helper(a: Integer, b: Integer, rev_values: &str) {
    let len = expected_range_len(&a, &b);
    assert_eq!(
        exhaustive_integer_inclusive_range(a.clone(), b.clone()).count(),
        len
    );
    let mut tail = exhaustive_integer_inclusive_range(a, b)
        .skip(len.saturating_sub(20))
        .collect_vec();
    tail.reverse();
    assert_eq!(tail.to_debug_string(), rev_values);
}

#[test]
fn test_exhaustive_integer_inclusive_range() {
    exhaustive_integer_inclusive_range_helper(&Integer::ZERO, &Integer::ZERO, "[0]");
    exhaustive_integer_inclusive_range_helper(&Integer::ZERO, &Integer::ONE, "[0, 1]");
    exhaustive_integer_inclusive_range_helper(
        &Integer::ZERO,
        &Integer::from(10),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]",
    );
    exhaustive_integer_inclusive_range_helper(
        &Integer::from(10),
        &Integer::from(20),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]",
    );
    exhaustive_integer_inclusive_range_helper(
        &Integer::from(-20),
        &Integer::from(-10),
        "[-10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]",
    );
    exhaustive_integer_inclusive_range_helper(
        &Integer::from(-100),
        &Integer::from(100),
        "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10]",
    );

    exhaustive_integer_inclusive_range_rev_helper(
        Integer::from(-20),
        Integer::from(-10),
        "[-20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10]",
    );
    exhaustive_integer_inclusive_range_rev_helper(
        Integer::from(-100),
        Integer::from(100),
        "[-100, 100, -99, 99, -98, 98, -97, 97, -96, 96, -95, 95, -94, 94, -93, 93, -92, 92, -91, \
        91]",
    );
}

#[test]
#[should_panic]
fn exhaustive_integer_inclusive_range_fail() {
    exhaustive_integer_inclusive_range(Integer::ONE, Integer::ZERO);
}
