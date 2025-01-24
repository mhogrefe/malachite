// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::geometric::get_geometric_random_signed_from_inclusive_range;
use malachite_base::num::random::VariableRangeGenerator;
use malachite_base::random::EXAMPLE_SEED;

fn get_geometric_random_signed_from_inclusive_range_helper(
    a: i32,
    b: i32,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    out: i32,
) {
    assert_eq!(
        get_geometric_random_signed_from_inclusive_range(
            &mut VariableRangeGenerator::new(EXAMPLE_SEED),
            a,
            b,
            abs_um_numerator,
            abs_um_denominator
        ),
        out
    );
}

#[test]
fn test_get_geometric_random_signed_from_inclusive_range() {
    get_geometric_random_signed_from_inclusive_range_helper(0, 0, 2, 1, 0);
    get_geometric_random_signed_from_inclusive_range_helper(0, 1, 2, 1, 1);
    get_geometric_random_signed_from_inclusive_range_helper(0, 10, 2, 1, 1);
    get_geometric_random_signed_from_inclusive_range_helper(0, 10, 10, 1, 2);
    get_geometric_random_signed_from_inclusive_range_helper(-10, 10, 2, 1, -3);
    get_geometric_random_signed_from_inclusive_range_helper(-10, 10, 10, 1, 8);
    get_geometric_random_signed_from_inclusive_range_helper(100, 110, 101, 1, 101);
    get_geometric_random_signed_from_inclusive_range_helper(100, 110, 500, 1, 109);
    get_geometric_random_signed_from_inclusive_range_helper(-110, -100, 101, 1, -101);
    get_geometric_random_signed_from_inclusive_range_helper(-110, -100, 500, 1, -109);
}

#[test]
#[should_panic]
fn get_geometric_random_signed_from_inclusive_range_fail_1() {
    get_geometric_random_signed_from_inclusive_range(
        &mut VariableRangeGenerator::new(EXAMPLE_SEED),
        1,
        0,
        1,
        2,
    );
}

#[test]
#[should_panic]
fn get_geometric_random_signed_from_inclusive_range_fail_2() {
    get_geometric_random_signed_from_inclusive_range(
        &mut VariableRangeGenerator::new(EXAMPLE_SEED),
        1,
        2,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn get_geometric_random_signed_from_inclusive_range_fail_3() {
    get_geometric_random_signed_from_inclusive_range(
        &mut VariableRangeGenerator::new(EXAMPLE_SEED),
        1,
        2,
        0,
        0,
    );
}

#[test]
#[should_panic]
fn get_geometric_random_signed_from_inclusive_range_fail_4() {
    get_geometric_random_signed_from_inclusive_range(
        &mut VariableRangeGenerator::new(EXAMPLE_SEED),
        1,
        2,
        1,
        2,
    );
}
