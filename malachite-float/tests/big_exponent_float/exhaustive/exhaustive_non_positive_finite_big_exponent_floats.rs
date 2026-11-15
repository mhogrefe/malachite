// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::big_exponent_float::exhaustive::*;
use malachite_float::test_util::big_exponent_float::exhaustive_big_exponent_floats_helper_helper;

#[test]
fn test_exhaustive_non_positive_finite_big_exponent_floats() {
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_non_positive_finite_big_exponent_floats(),
        50,
        &[
            "0.0", "-1.0", "-2.0", "-1.5", "-3.0", "-0.50", "-4.0", "-0.75", "-6.0", "-1.2",
            "-2.5", "-1.8", "-3.5", "-0.62", "-5.0", "-0.88", "-7.0", "-0.25", "-8.0", "-0.38",
            "-12.0", "-0.12", "-16.0", "-0.19", "-24.0", "-0.31", "-10.0", "-0.44", "-14.0",
            "-0.16", "-20.0", "-0.22", "-28.0", "-1.12", "-2.25", "-1.38", "-2.75", "-0.562",
            "-4.50", "-0.688", "-5.50", "-1.62", "-3.25", "-1.88", "-3.75", "-0.812", "-6.50",
            "-0.938", "-7.50", "-0.281",
        ],
        &[
            "0x0.0",
            "-0x1.0",
            "-0x2.0",
            "-0x1.8",
            "-0x3.0",
            "-0x0.8",
            "-0x4.0",
            "-0x0.c",
            "-0x6.0",
            "-0x1.4",
            "-0x2.8",
            "-0x1.c",
            "-0x3.8",
            "-0x0.a",
            "-0x5.0",
            "-0x0.e",
            "-0x7.0",
            "-0x0.4",
            "-0x8.0",
            "-0x0.6",
            "-0xc.0",
            "-0x0.2",
            "-0x1.0E+1",
            "-0x0.3",
            "-0x18.0",
            "-0x0.5",
            "-0xa.0",
            "-0x0.7",
            "-0xe.0",
            "-0x0.28",
            "-0x14.0",
            "-0x0.38",
            "-0x1c.0",
            "-0x1.2",
            "-0x2.4",
            "-0x1.6",
            "-0x2.c",
            "-0x0.9",
            "-0x4.8",
            "-0x0.b",
            "-0x5.8",
            "-0x1.a",
            "-0x3.4",
            "-0x1.e",
            "-0x3.c",
            "-0x0.d",
            "-0x6.8",
            "-0x0.f",
            "-0x7.8",
            "-0x0.48",
        ],
    );
}
