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
fn test_exhaustive_positive_big_exponent_floats_with_sci_exponent() {
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent(Integer::ZERO),
        50,
        &[
            "0x1.0", "0x1.8", "0x1.4", "0x1.c", "0x1.2", "0x1.6", "0x1.a", "0x1.e", "0x1.1",
            "0x1.3", "0x1.5", "0x1.7", "0x1.9", "0x1.b", "0x1.d", "0x1.f", "0x1.08", "0x1.18",
            "0x1.28", "0x1.38", "0x1.48", "0x1.58", "0x1.68", "0x1.78", "0x1.88", "0x1.98",
            "0x1.a8", "0x1.b8", "0x1.c8", "0x1.d8", "0x1.e8", "0x1.f8", "0x1.04", "0x1.0c",
            "0x1.14", "0x1.1c", "0x1.24", "0x1.2c", "0x1.34", "0x1.3c", "0x1.44", "0x1.4c",
            "0x1.54", "0x1.5c", "0x1.64", "0x1.6c", "0x1.74", "0x1.7c", "0x1.84", "0x1.8c",
        ],
    );
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent(Integer::from(-5)),
        50,
        &[
            "0x0.08", "0x0.0c", "0x0.0a", "0x0.0e", "0x0.09", "0x0.0b", "0x0.0d", "0x0.0f",
            "0x0.088", "0x0.098", "0x0.0a8", "0x0.0b8", "0x0.0c8", "0x0.0d8", "0x0.0e8", "0x0.0f8",
            "0x0.084", "0x0.08c", "0x0.094", "0x0.09c", "0x0.0a4", "0x0.0ac", "0x0.0b4", "0x0.0bc",
            "0x0.0c4", "0x0.0cc", "0x0.0d4", "0x0.0dc", "0x0.0e4", "0x0.0ec", "0x0.0f4", "0x0.0fc",
            "0x0.082", "0x0.086", "0x0.08a", "0x0.08e", "0x0.092", "0x0.096", "0x0.09a", "0x0.09e",
            "0x0.0a2", "0x0.0a6", "0x0.0aa", "0x0.0ae", "0x0.0b2", "0x0.0b6", "0x0.0ba", "0x0.0be",
            "0x0.0c2", "0x0.0c6",
        ],
    );
    exhaustive_big_exponent_floats_helper_helper(
        exhaustive_positive_big_exponent_floats_with_sci_exponent(
            Integer::from_str("-1000000000000000000000").unwrap(),
        ),
        50,
        &[
            "0x1.0E-250000000000000000000",
            "0x1.8E-250000000000000000000",
            "0x1.4E-250000000000000000000",
            "0x1.cE-250000000000000000000",
            "0x1.2E-250000000000000000000",
            "0x1.6E-250000000000000000000",
            "0x1.aE-250000000000000000000",
            "0x1.eE-250000000000000000000",
            "0x1.1E-250000000000000000000",
            "0x1.3E-250000000000000000000",
            "0x1.5E-250000000000000000000",
            "0x1.7E-250000000000000000000",
            "0x1.9E-250000000000000000000",
            "0x1.bE-250000000000000000000",
            "0x1.dE-250000000000000000000",
            "0x1.fE-250000000000000000000",
            "0x1.08E-250000000000000000000",
            "0x1.18E-250000000000000000000",
            "0x1.28E-250000000000000000000",
            "0x1.38E-250000000000000000000",
            "0x1.48E-250000000000000000000",
            "0x1.58E-250000000000000000000",
            "0x1.68E-250000000000000000000",
            "0x1.78E-250000000000000000000",
            "0x1.88E-250000000000000000000",
            "0x1.98E-250000000000000000000",
            "0x1.a8E-250000000000000000000",
            "0x1.b8E-250000000000000000000",
            "0x1.c8E-250000000000000000000",
            "0x1.d8E-250000000000000000000",
            "0x1.e8E-250000000000000000000",
            "0x1.f8E-250000000000000000000",
            "0x1.04E-250000000000000000000",
            "0x1.0cE-250000000000000000000",
            "0x1.14E-250000000000000000000",
            "0x1.1cE-250000000000000000000",
            "0x1.24E-250000000000000000000",
            "0x1.2cE-250000000000000000000",
            "0x1.34E-250000000000000000000",
            "0x1.3cE-250000000000000000000",
            "0x1.44E-250000000000000000000",
            "0x1.4cE-250000000000000000000",
            "0x1.54E-250000000000000000000",
            "0x1.5cE-250000000000000000000",
            "0x1.64E-250000000000000000000",
            "0x1.6cE-250000000000000000000",
            "0x1.74E-250000000000000000000",
            "0x1.7cE-250000000000000000000",
            "0x1.84E-250000000000000000000",
            "0x1.8cE-250000000000000000000",
        ],
    );
}
