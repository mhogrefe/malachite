// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::random::get_weighted_random_bool;
use malachite_base::num::random::VariableRangeGenerator;
use malachite_base::random::EXAMPLE_SEED;

fn get_weighted_random_bool_helper(n: u64, d: u64, out: bool) {
    assert_eq!(
        get_weighted_random_bool(&mut VariableRangeGenerator::new(EXAMPLE_SEED), n, d),
        out
    );
}

#[test]
fn test_get_weighted_random_bool() {
    get_weighted_random_bool_helper(0, 1, false);
    get_weighted_random_bool_helper(1, 1, true);
    get_weighted_random_bool_helper(1, 2, false);
    get_weighted_random_bool_helper(1, 100, false);
    get_weighted_random_bool_helper(99, 100, true);
}

#[test]
#[should_panic]
fn get_weighted_random_bool_fail_1() {
    get_weighted_random_bool(&mut VariableRangeGenerator::new(EXAMPLE_SEED), 0, 0);
}

#[test]
#[should_panic]
fn get_weighted_random_bool_fail_2() {
    get_weighted_random_bool(&mut VariableRangeGenerator::new(EXAMPLE_SEED), 1, 0);
}

#[test]
#[should_panic]
fn get_weighted_random_bool_fail_3() {
    get_weighted_random_bool(&mut VariableRangeGenerator::new(EXAMPLE_SEED), 2, 1);
}
