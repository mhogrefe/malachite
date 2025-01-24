// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::exhaustive::exhaustive_floats_with_precision;
use malachite_float::test_util::exhaustive::exhaustive_floats_helper_helper_with_limit;
use std::panic::catch_unwind;

fn exhaustive_floats_with_precision_helper(precision: u64, out: &[&str], out_hex: &[&str]) {
    exhaustive_floats_helper_helper_with_limit(
        20,
        exhaustive_floats_with_precision(precision),
        out,
        out_hex,
    );
}

#[test]
fn test_exhaustive_floats_with_precision() {
    exhaustive_floats_with_precision_helper(
        1,
        &[
            "1.0", "-1.0", "2.0", "-2.0", "0.5", "-0.5", "0.2", "-0.2", "4.0", "-4.0", "8.0",
            "-8.0", "0.1", "-0.1", "3.0e1", "-3.0e1", "2.0e1", "-2.0e1", "0.06", "-0.06",
        ],
        &[
            "0x1.0#1",
            "-0x1.0#1",
            "0x2.0#1",
            "-0x2.0#1",
            "0x0.8#1",
            "-0x0.8#1",
            "0x0.4#1",
            "-0x0.4#1",
            "0x4.0#1",
            "-0x4.0#1",
            "0x8.0#1",
            "-0x8.0#1",
            "0x0.2#1",
            "-0x0.2#1",
            "0x2.0E+1#1",
            "-0x2.0E+1#1",
            "0x1.0E+1#1",
            "-0x1.0E+1#1",
            "0x0.1#1",
            "-0x0.1#1",
        ],
    );
    exhaustive_floats_with_precision_helper(
        2,
        &[
            "1.0", "-1.0", "2.0", "-2.0", "1.5", "-1.5", "0.5", "-0.5", "3.0", "-3.0", "0.8",
            "-0.8", "4.0", "-4.0", "0.1", "-0.1", "6.0", "-6.0", "0.2", "-0.2",
        ],
        &[
            "0x1.0#2", "-0x1.0#2", "0x2.0#2", "-0x2.0#2", "0x1.8#2", "-0x1.8#2", "0x0.8#2",
            "-0x0.8#2", "0x3.0#2", "-0x3.0#2", "0x0.c#2", "-0x0.c#2", "0x4.0#2", "-0x4.0#2",
            "0x0.2#2", "-0x0.2#2", "0x6.0#2", "-0x6.0#2", "0x0.4#2", "-0x0.4#2",
        ],
    );
    exhaustive_floats_with_precision_helper(
        10,
        &[
            "1.0", "-1.0", "2.0", "-2.0", "1.002", "-1.002", "0.5", "-0.5", "1.004", "-1.004",
            "2.004", "-2.004", "1.006", "-1.006", "4.0", "-4.0", "1.008", "-1.008", "2.008",
            "-2.008",
        ],
        &[
            "0x1.000#10",
            "-0x1.000#10",
            "0x2.00#10",
            "-0x2.00#10",
            "0x1.008#10",
            "-0x1.008#10",
            "0x0.800#10",
            "-0x0.800#10",
            "0x1.010#10",
            "-0x1.010#10",
            "0x2.01#10",
            "-0x2.01#10",
            "0x1.018#10",
            "-0x1.018#10",
            "0x4.00#10",
            "-0x4.00#10",
            "0x1.020#10",
            "-0x1.020#10",
            "0x2.02#10",
            "-0x2.02#10",
        ],
    );
    exhaustive_floats_with_precision_helper(
        100,
        &[
            "1.0",
            "-1.0",
            "2.0",
            "-2.0",
            "1.000000000000000000000000000002",
            "-1.000000000000000000000000000002",
            "0.5",
            "-0.5",
            "1.000000000000000000000000000003",
            "-1.000000000000000000000000000003",
            "2.000000000000000000000000000003",
            "-2.000000000000000000000000000003",
            "1.000000000000000000000000000005",
            "-1.000000000000000000000000000005",
            "4.0",
            "-4.0",
            "1.000000000000000000000000000006",
            "-1.000000000000000000000000000006",
            "2.000000000000000000000000000006",
            "-2.000000000000000000000000000006",
        ],
        &[
            "0x1.0000000000000000000000000#100",
            "-0x1.0000000000000000000000000#100",
            "0x2.0000000000000000000000000#100",
            "-0x2.0000000000000000000000000#100",
            "0x1.0000000000000000000000002#100",
            "-0x1.0000000000000000000000002#100",
            "0x0.8000000000000000000000000#100",
            "-0x0.8000000000000000000000000#100",
            "0x1.0000000000000000000000004#100",
            "-0x1.0000000000000000000000004#100",
            "0x2.0000000000000000000000004#100",
            "-0x2.0000000000000000000000004#100",
            "0x1.0000000000000000000000006#100",
            "-0x1.0000000000000000000000006#100",
            "0x4.0000000000000000000000000#100",
            "-0x4.0000000000000000000000000#100",
            "0x1.0000000000000000000000008#100",
            "-0x1.0000000000000000000000008#100",
            "0x2.0000000000000000000000008#100",
            "-0x2.0000000000000000000000008#100",
        ],
    );
}

#[test]
fn exhaustive_floats_with_precision_fail() {
    assert_panic!(exhaustive_floats_with_precision(0));
}
