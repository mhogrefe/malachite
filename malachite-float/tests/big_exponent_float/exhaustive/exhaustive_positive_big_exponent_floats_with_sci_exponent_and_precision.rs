// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_float::big_exponent_float::exhaustive::*;
use malachite_float::test_util::big_exponent_float::exhaustive_big_exponent_floats_helper_helper;
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision() {
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(Integer::ZERO, 1),
        50,
        &["0x1.0"],
    );
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(Integer::ZERO, 4),
        50,
        &["0x1.2", "0x1.6", "0x1.a", "0x1.e"],
    );
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(
            Integer::from(-3),
            6,
        ),
        50,
        &[
            "0x0.21", "0x0.23", "0x0.25", "0x0.27", "0x0.29", "0x0.2b", "0x0.2d", "0x0.2f",
            "0x0.31", "0x0.33", "0x0.35", "0x0.37", "0x0.39", "0x0.3b", "0x0.3d", "0x0.3f",
        ],
    );
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(
            Integer::from_str("1000000000000000000000").unwrap(),
            3,
        ),
        50,
        &["0x1.4E+250000000000000000000", "0x1.cE+250000000000000000000"],
    );
}

#[test]
#[should_panic]
fn exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision_fail_1() {
    exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(Integer::ZERO, 0);
}
