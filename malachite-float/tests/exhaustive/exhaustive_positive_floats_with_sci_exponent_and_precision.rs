// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::exhaustive::exhaustive_positive_floats_with_sci_exponent_and_precision;
use malachite_float::test_util::exhaustive::exhaustive_floats_helper_helper_with_limit;
use std::panic::catch_unwind;

fn exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
    sci_exponent: i32,
    prec: u64,
    out: &[&str],
    out_hex: &[&str],
) {
    exhaustive_floats_helper_helper_with_limit(
        20,
        exhaustive_positive_floats_with_sci_exponent_and_precision(sci_exponent, prec),
        out,
        out_hex,
    );
}

#[test]
fn test_exhaustive_positive_floats_with_sci_exponent_and_precision() {
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(0, 1, &["1.0"], &["0x1.0#1"]);
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        0,
        2,
        &["1.0", "1.5"],
        &["0x1.0#2", "0x1.8#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        0,
        3,
        &["1.0", "1.2", "1.5", "1.8"],
        &["0x1.0#3", "0x1.4#3", "0x1.8#3", "0x1.c#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        0,
        4,
        &["1.0", "1.1", "1.2", "1.4", "1.5", "1.6", "1.8", "1.9"],
        &["0x1.0#4", "0x1.2#4", "0x1.4#4", "0x1.6#4", "0x1.8#4", "0x1.a#4", "0x1.c#4", "0x1.e#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        0,
        5,
        &[
            "1.0", "1.06", "1.12", "1.19", "1.25", "1.3", "1.38", "1.44", "1.5", "1.56", "1.62",
            "1.7", "1.75", "1.81", "1.88", "1.94",
        ],
        &[
            "0x1.0#5", "0x1.1#5", "0x1.2#5", "0x1.3#5", "0x1.4#5", "0x1.5#5", "0x1.6#5", "0x1.7#5",
            "0x1.8#5", "0x1.9#5", "0x1.a#5", "0x1.b#5", "0x1.c#5", "0x1.d#5", "0x1.e#5", "0x1.f#5",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(1, 1, &["2.0"], &["0x2.0#1"]);
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        1,
        2,
        &["2.0", "3.0"],
        &["0x2.0#2", "0x3.0#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        1,
        3,
        &["2.0", "2.5", "3.0", "3.5"],
        &["0x2.0#3", "0x2.8#3", "0x3.0#3", "0x3.8#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        1,
        4,
        &["2.0", "2.2", "2.5", "2.8", "3.0", "3.2", "3.5", "3.8"],
        &["0x2.0#4", "0x2.4#4", "0x2.8#4", "0x2.c#4", "0x3.0#4", "0x3.4#4", "0x3.8#4", "0x3.c#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        1,
        5,
        &[
            "2.0", "2.1", "2.2", "2.4", "2.5", "2.6", "2.8", "2.9", "3.0", "3.1", "3.2", "3.4",
            "3.5", "3.6", "3.8", "3.9",
        ],
        &[
            "0x2.0#5", "0x2.2#5", "0x2.4#5", "0x2.6#5", "0x2.8#5", "0x2.a#5", "0x2.c#5", "0x2.e#5",
            "0x3.0#5", "0x3.2#5", "0x3.4#5", "0x3.6#5", "0x3.8#5", "0x3.a#5", "0x3.c#5", "0x3.e#5",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        1,
        &["2.0e1"],
        &["0x1.0E+1#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        2,
        &["16.0", "24.0"],
        &["0x10.0#2", "0x18.0#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        3,
        &["16.0", "20.0", "24.0", "28.0"],
        &["0x10.0#3", "0x14.0#3", "0x18.0#3", "0x1c.0#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        4,
        &["16.0", "18.0", "20.0", "22.0", "24.0", "26.0", "28.0", "30.0"],
        &[
            "0x10.0#4", "0x12.0#4", "0x14.0#4", "0x16.0#4", "0x18.0#4", "0x1a.0#4", "0x1c.0#4",
            "0x1e.0#4",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        5,
        &[
            "16.0", "17.0", "18.0", "19.0", "20.0", "21.0", "22.0", "23.0", "24.0", "25.0", "26.0",
            "27.0", "28.0", "29.0", "30.0", "31.0",
        ],
        &[
            "0x10.0#5", "0x11.0#5", "0x12.0#5", "0x13.0#5", "0x14.0#5", "0x15.0#5", "0x16.0#5",
            "0x17.0#5", "0x18.0#5", "0x19.0#5", "0x1a.0#5", "0x1b.0#5", "0x1c.0#5", "0x1d.0#5",
            "0x1e.0#5", "0x1f.0#5",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        1,
        &["1.0e30"],
        &["0x1.0E+25#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        2,
        &["1.0e30", "2.0e30"],
        &["0x1.0E+25#2", "0x1.8E+25#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        3,
        &["1.3e30", "1.6e30", "1.9e30", "2.2e30"],
        &["0x1.0E+25#3", "0x1.4E+25#3", "0x1.8E+25#3", "0x1.cE+25#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        4,
        &["1.3e30", "1.4e30", "1.6e30", "1.7e30", "1.9e30", "2.1e30", "2.2e30", "2.4e30"],
        &[
            "0x1.0E+25#4",
            "0x1.2E+25#4",
            "0x1.4E+25#4",
            "0x1.6E+25#4",
            "0x1.8E+25#4",
            "0x1.aE+25#4",
            "0x1.cE+25#4",
            "0x1.eE+25#4",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        5,
        &[
            "1.27e30", "1.35e30", "1.4e30", "1.5e30", "1.6e30", "1.66e30", "1.74e30", "1.8e30",
            "1.9e30", "2.0e30", "2.06e30", "2.14e30", "2.2e30", "2.3e30", "2.4e30", "2.46e30",
        ],
        &[
            "0x1.0E+25#5",
            "0x1.1E+25#5",
            "0x1.2E+25#5",
            "0x1.3E+25#5",
            "0x1.4E+25#5",
            "0x1.5E+25#5",
            "0x1.6E+25#5",
            "0x1.7E+25#5",
            "0x1.8E+25#5",
            "0x1.9E+25#5",
            "0x1.aE+25#5",
            "0x1.bE+25#5",
            "0x1.cE+25#5",
            "0x1.dE+25#5",
            "0x1.eE+25#5",
            "0x1.fE+25#5",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        30,
        &[
            "1.2676506e30",
            "1.267650603e30",
            "1.267650605e30",
            "1.267650607e30",
            "1.26765061e30",
            "1.267650612e30",
            "1.267650614e30",
            "1.267650617e30",
            "1.267650619e30",
            "1.267650621e30",
            "1.267650624e30",
            "1.267650626e30",
            "1.267650629e30",
            "1.267650631e30",
            "1.267650633e30",
            "1.267650636e30",
            "1.267650638e30",
            "1.26765064e30",
            "1.267650643e30",
            "1.267650645e30",
        ],
        &[
            "0x1.00000000E+25#30",
            "0x1.00000008E+25#30",
            "0x1.00000010E+25#30",
            "0x1.00000018E+25#30",
            "0x1.00000020E+25#30",
            "0x1.00000028E+25#30",
            "0x1.00000030E+25#30",
            "0x1.00000038E+25#30",
            "0x1.00000040E+25#30",
            "0x1.00000048E+25#30",
            "0x1.00000050E+25#30",
            "0x1.00000058E+25#30",
            "0x1.00000060E+25#30",
            "0x1.00000068E+25#30",
            "0x1.00000070E+25#30",
            "0x1.00000078E+25#30",
            "0x1.00000080E+25#30",
            "0x1.00000088E+25#30",
            "0x1.00000090E+25#30",
            "0x1.00000098E+25#30",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        1,
        &["0.5"],
        &["0x0.8#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        2,
        &["0.5", "0.8"],
        &["0x0.8#2", "0x0.c#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        3,
        &["0.5", "0.6", "0.8", "0.9"],
        &["0x0.8#3", "0x0.a#3", "0x0.c#3", "0x0.e#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        4,
        &["0.5", "0.56", "0.62", "0.7", "0.75", "0.81", "0.88", "0.94"],
        &["0x0.8#4", "0x0.9#4", "0x0.a#4", "0x0.b#4", "0x0.c#4", "0x0.d#4", "0x0.e#4", "0x0.f#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        5,
        &[
            "0.5", "0.53", "0.56", "0.59", "0.62", "0.66", "0.69", "0.72", "0.75", "0.78", "0.81",
            "0.84", "0.88", "0.91", "0.94", "0.97",
        ],
        &[
            "0x0.80#5", "0x0.88#5", "0x0.90#5", "0x0.98#5", "0x0.a0#5", "0x0.a8#5", "0x0.b0#5",
            "0x0.b8#5", "0x0.c0#5", "0x0.c8#5", "0x0.d0#5", "0x0.d8#5", "0x0.e0#5", "0x0.e8#5",
            "0x0.f0#5", "0x0.f8#5",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -10,
        1,
        &["0.001"],
        &["0x0.004#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -10,
        2,
        &["0.00098", "0.0015"],
        &["0x0.004#2", "0x0.006#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -10,
        3,
        &["0.00098", "0.0012", "0.0015", "0.0017"],
        &["0x0.004#3", "0x0.005#3", "0x0.006#3", "0x0.007#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -10,
        4,
        &["0.00098", "0.0011", "0.0012", "0.0013", "0.0015", "0.0016", "0.0017", "0.0018"],
        &[
            "0x0.0040#4",
            "0x0.0048#4",
            "0x0.0050#4",
            "0x0.0058#4",
            "0x0.0060#4",
            "0x0.0068#4",
            "0x0.0070#4",
            "0x0.0078#4",
        ],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -10,
        5,
        &[
            "0.00098", "0.001", "0.0011", "0.00116", "0.00122", "0.00128", "0.00134", "0.0014",
            "0.00146", "0.00153", "0.00159", "0.00165", "0.0017", "0.00177", "0.00183", "0.0019",
        ],
        &[
            "0x0.0040#5",
            "0x0.0044#5",
            "0x0.0048#5",
            "0x0.004c#5",
            "0x0.0050#5",
            "0x0.0054#5",
            "0x0.0058#5",
            "0x0.005c#5",
            "0x0.0060#5",
            "0x0.0064#5",
            "0x0.0068#5",
            "0x0.006c#5",
            "0x0.0070#5",
            "0x0.0074#5",
            "0x0.0078#5",
            "0x0.007c#5",
        ],
    );
}

#[test]
fn exhaustive_positive_floats_with_sci_exponent_and_precision_fail() {
    assert_panic!(exhaustive_positive_floats_with_sci_exponent_and_precision(
        1, 0
    ));
}
