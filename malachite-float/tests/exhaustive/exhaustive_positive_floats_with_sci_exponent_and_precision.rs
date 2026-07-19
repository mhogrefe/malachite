// Copyright © 2026 Mikhail Hogrefe
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
        &["1.00", "1.12", "1.25", "1.38", "1.50", "1.62", "1.75", "1.88"],
        &["0x1.0#4", "0x1.2#4", "0x1.4#4", "0x1.6#4", "0x1.8#4", "0x1.a#4", "0x1.c#4", "0x1.e#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        0,
        5,
        &[
            "1.00", "1.06", "1.12", "1.19", "1.25", "1.31", "1.38", "1.44", "1.50", "1.56", "1.62",
            "1.69", "1.75", "1.81", "1.88", "1.94",
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
        &["2.00", "2.25", "2.50", "2.75", "3.00", "3.25", "3.50", "3.75"],
        &["0x2.0#4", "0x2.4#4", "0x2.8#4", "0x2.c#4", "0x3.0#4", "0x3.4#4", "0x3.8#4", "0x3.c#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        1,
        5,
        &[
            "2.00", "2.12", "2.25", "2.38", "2.50", "2.62", "2.75", "2.88", "3.00", "3.12", "3.25",
            "3.38", "3.50", "3.62", "3.75", "3.88",
        ],
        &[
            "0x2.0#5", "0x2.2#5", "0x2.4#5", "0x2.6#5", "0x2.8#5", "0x2.a#5", "0x2.c#5", "0x2.e#5",
            "0x3.0#5", "0x3.2#5", "0x3.4#5", "0x3.6#5", "0x3.8#5", "0x3.a#5", "0x3.c#5", "0x3.e#5",
        ],
    );

    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        4,
        1,
        &["16.0"],
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
        &["1.3e30"],
        &["0x1.0E+25#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        100,
        2,
        &["1.3e30", "1.9e30"],
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
        &["1.27e30", "1.43e30", "1.58e30", "1.74e30", "1.90e30", "2.06e30", "2.22e30", "2.38e30"],
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
            "1.27e30", "1.35e30", "1.43e30", "1.51e30", "1.58e30", "1.66e30", "1.74e30", "1.82e30",
            "1.90e30", "1.98e30", "2.06e30", "2.14e30", "2.22e30", "2.30e30", "2.38e30", "2.46e30",
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
            "1.2676506002e30",
            "1.2676506026e30",
            "1.2676506050e30",
            "1.2676506073e30",
            "1.2676506097e30",
            "1.2676506120e30",
            "1.2676506144e30",
            "1.2676506168e30",
            "1.2676506191e30",
            "1.2676506215e30",
            "1.2676506238e30",
            "1.2676506262e30",
            "1.2676506286e30",
            "1.2676506309e30",
            "1.2676506333e30",
            "1.2676506356e30",
            "1.2676506380e30",
            "1.2676506404e30",
            "1.2676506427e30",
            "1.2676506451e30",
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
        &["0.50"],
        &["0x0.8#1"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        2,
        &["0.50", "0.75"],
        &["0x0.8#2", "0x0.c#2"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        3,
        &["0.50", "0.62", "0.75", "0.88"],
        &["0x0.8#3", "0x0.a#3", "0x0.c#3", "0x0.e#3"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        4,
        &["0.500", "0.562", "0.625", "0.688", "0.750", "0.812", "0.875", "0.938"],
        &["0x0.8#4", "0x0.9#4", "0x0.a#4", "0x0.b#4", "0x0.c#4", "0x0.d#4", "0x0.e#4", "0x0.f#4"],
    );
    exhaustive_positive_floats_with_sci_exponent_and_precision_helper(
        -1,
        5,
        &[
            "0.500", "0.531", "0.562", "0.594", "0.625", "0.656", "0.688", "0.719", "0.750",
            "0.781", "0.812", "0.844", "0.875", "0.906", "0.938", "0.969",
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
        &["0.00098"],
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
        &["0.000977", "0.00110", "0.00122", "0.00134", "0.00146", "0.00159", "0.00171", "0.00183"],
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
            "0.000977", "0.00104", "0.00110", "0.00116", "0.00122", "0.00128", "0.00134",
            "0.00140", "0.00146", "0.00153", "0.00159", "0.00165", "0.00171", "0.00177", "0.00183",
            "0.00189",
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
