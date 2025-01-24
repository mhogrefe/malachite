// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::exhaustive::exhaustive_positive_floats_with_sci_exponent;
use malachite_float::test_util::exhaustive::exhaustive_floats_helper_helper_with_limit;

fn exhaustive_positive_floats_with_sci_exponent_helper(
    sci_exponent: i32,
    out: &[&str],
    out_hex: &[&str],
) {
    exhaustive_floats_helper_helper_with_limit(
        20,
        exhaustive_positive_floats_with_sci_exponent(sci_exponent),
        out,
        out_hex,
    );
}

#[test]
fn test_exhaustive_positive_floats_with_sci_exponent() {
    exhaustive_positive_floats_with_sci_exponent_helper(
        0,
        &[
            "1.0", "1.0", "1.5", "1.0", "1.2", "1.5", "1.8", "1.0", "1.1", "1.2", "1.4", "1.5",
            "1.6", "1.8", "1.9", "1.0", "1.06", "1.12", "1.19", "1.25",
        ],
        &[
            "0x1.0#1", "0x1.0#2", "0x1.8#2", "0x1.0#3", "0x1.4#3", "0x1.8#3", "0x1.c#3", "0x1.0#4",
            "0x1.2#4", "0x1.4#4", "0x1.6#4", "0x1.8#4", "0x1.a#4", "0x1.c#4", "0x1.e#4", "0x1.0#5",
            "0x1.1#5", "0x1.2#5", "0x1.3#5", "0x1.4#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        1,
        &[
            "2.0", "2.0", "3.0", "2.0", "2.5", "3.0", "3.5", "2.0", "2.2", "2.5", "2.8", "3.0",
            "3.2", "3.5", "3.8", "2.0", "2.1", "2.2", "2.4", "2.5",
        ],
        &[
            "0x2.0#1", "0x2.0#2", "0x3.0#2", "0x2.0#3", "0x2.8#3", "0x3.0#3", "0x3.8#3", "0x2.0#4",
            "0x2.4#4", "0x2.8#4", "0x2.c#4", "0x3.0#4", "0x3.4#4", "0x3.8#4", "0x3.c#4", "0x2.0#5",
            "0x2.2#5", "0x2.4#5", "0x2.6#5", "0x2.8#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        2,
        &[
            "4.0", "4.0", "6.0", "4.0", "5.0", "6.0", "7.0", "4.0", "4.5", "5.0", "5.5", "6.0",
            "6.5", "7.0", "7.5", "4.0", "4.2", "4.5", "4.8", "5.0",
        ],
        &[
            "0x4.0#1", "0x4.0#2", "0x6.0#2", "0x4.0#3", "0x5.0#3", "0x6.0#3", "0x7.0#3", "0x4.0#4",
            "0x4.8#4", "0x5.0#4", "0x5.8#4", "0x6.0#4", "0x6.8#4", "0x7.0#4", "0x7.8#4", "0x4.0#5",
            "0x4.4#5", "0x4.8#5", "0x4.c#5", "0x5.0#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        10,
        &[
            "1.0e3", "1.0e3", "1.5e3", "1.0e3", "1.3e3", "1.5e3", "1.8e3", "1.0e3", "1.2e3",
            "1.3e3", "1.4e3", "1.5e3", "1.7e3", "1.8e3", "1.9e3", "1.0e3", "1.1e3", "1.15e3",
            "1.22e3", "1.28e3",
        ],
        &[
            "0x4.0E+2#1",
            "0x4.0E+2#2",
            "0x6.0E+2#2",
            "0x4.0E+2#3",
            "0x5.0E+2#3",
            "0x6.0E+2#3",
            "0x7.0E+2#3",
            "0x4.0E+2#4",
            "0x4.8E+2#4",
            "0x5.0E+2#4",
            "0x5.8E+2#4",
            "0x6.0E+2#4",
            "0x6.8E+2#4",
            "0x7.0E+2#4",
            "0x7.8E+2#4",
            "0x4.0E+2#5",
            "0x4.4E+2#5",
            "0x4.8E+2#5",
            "0x4.cE+2#5",
            "0x5.0E+2#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        -1,
        &[
            "0.5", "0.5", "0.8", "0.5", "0.6", "0.8", "0.9", "0.5", "0.56", "0.62", "0.7", "0.75",
            "0.81", "0.88", "0.94", "0.5", "0.53", "0.56", "0.59", "0.62",
        ],
        &[
            "0x0.8#1", "0x0.8#2", "0x0.c#2", "0x0.8#3", "0x0.a#3", "0x0.c#3", "0x0.e#3", "0x0.8#4",
            "0x0.9#4", "0x0.a#4", "0x0.b#4", "0x0.c#4", "0x0.d#4", "0x0.e#4", "0x0.f#4",
            "0x0.80#5", "0x0.88#5", "0x0.90#5", "0x0.98#5", "0x0.a0#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        -2,
        &[
            "0.2", "0.2", "0.4", "0.25", "0.3", "0.38", "0.44", "0.25", "0.28", "0.31", "0.34",
            "0.38", "0.41", "0.44", "0.47", "0.25", "0.27", "0.28", "0.3", "0.31",
        ],
        &[
            "0x0.4#1", "0x0.4#2", "0x0.6#2", "0x0.4#3", "0x0.5#3", "0x0.6#3", "0x0.7#3",
            "0x0.40#4", "0x0.48#4", "0x0.50#4", "0x0.58#4", "0x0.60#4", "0x0.68#4", "0x0.70#4",
            "0x0.78#4", "0x0.40#5", "0x0.44#5", "0x0.48#5", "0x0.4c#5", "0x0.50#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_helper(
        -10,
        &[
            "0.001", "0.00098", "0.0015", "0.00098", "0.0012", "0.0015", "0.0017", "0.00098",
            "0.0011", "0.0012", "0.0013", "0.0015", "0.0016", "0.0017", "0.0018", "0.00098",
            "0.001", "0.0011", "0.00116", "0.00122",
        ],
        &[
            "0x0.004#1",
            "0x0.004#2",
            "0x0.006#2",
            "0x0.004#3",
            "0x0.005#3",
            "0x0.006#3",
            "0x0.007#3",
            "0x0.0040#4",
            "0x0.0048#4",
            "0x0.0050#4",
            "0x0.0058#4",
            "0x0.0060#4",
            "0x0.0068#4",
            "0x0.0070#4",
            "0x0.0078#4",
            "0x0.0040#5",
            "0x0.0044#5",
            "0x0.0048#5",
            "0x0.004c#5",
            "0x0.0050#5",
        ],
    );
}
