// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::big_exponent_float::exhaustive::exhaustive_nonzero_finite_big_exponent_floats;
use malachite_float::test_util::big_exponent_float::exhaustive_big_exponent_floats_helper_helper;

#[test]
fn test_exhaustive_nonzero_finite_big_exponent_floats() {
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_nonzero_finite_big_exponent_floats(),
        50,
        &[
            "0x1.0",
            "-0x1.0",
            "0x2.0",
            "-0x2.0",
            "0x1.8",
            "-0x1.8",
            "0x3.0",
            "-0x3.0",
            "0x0.8",
            "-0x0.8",
            "0x4.0",
            "-0x4.0",
            "0x0.c",
            "-0x0.c",
            "0x6.0",
            "-0x6.0",
            "0x1.4",
            "-0x1.4",
            "0x2.8",
            "-0x2.8",
            "0x1.c",
            "-0x1.c",
            "0x3.8",
            "-0x3.8",
            "0x0.a",
            "-0x0.a",
            "0x5.0",
            "-0x5.0",
            "0x0.e",
            "-0x0.e",
            "0x7.0",
            "-0x7.0",
            "0x0.4",
            "-0x0.4",
            "0x8.0",
            "-0x8.0",
            "0x0.6",
            "-0x0.6",
            "0xc.0",
            "-0xc.0",
            "0x0.2",
            "-0x0.2",
            "0x1.0E+1",
            "-0x1.0E+1",
            "0x0.3",
            "-0x0.3",
            "0x18.0",
            "-0x18.0",
            "0x0.5",
            "-0x0.5",
        ],
    );
}
