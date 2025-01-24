// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::exhaustive::exhaustive_nonzero_finite_floats;
use malachite_float::test_util::exhaustive::exhaustive_floats_helper_helper_with_limit;

#[test]
fn test_exhaustive_nonzero_finite_floats() {
    exhaustive_floats_helper_helper_with_limit(
        100,
        exhaustive_nonzero_finite_floats(),
        &[
            "1.0", "-1.0", "2.0", "-2.0", "1.0", "-1.0", "0.5", "-0.5", "1.5", "-1.5", "2.0",
            "-2.0", "1.0", "-1.0", "4.0", "-4.0", "1.2", "-1.2", "3.0", "-3.0", "1.5", "-1.5",
            "0.5", "-0.5", "1.8", "-1.8", "2.0", "-2.0", "1.0", "-1.0", "0.2", "-0.2", "1.1",
            "-1.1", "2.5", "-2.5", "1.2", "-1.2", "0.8", "-0.8", "1.4", "-1.4", "3.0", "-3.0",
            "1.5", "-1.5", "4.0", "-4.0", "1.6", "-1.6", "3.5", "-3.5", "1.8", "-1.8", "0.5",
            "-0.5", "1.9", "-1.9", "2.0", "-2.0", "1.0", "-1.0", "8.0", "-8.0", "1.06", "-1.06",
            "2.2", "-2.2", "1.12", "-1.12", "0.6", "-0.6", "1.19", "-1.19", "2.5", "-2.5", "1.25",
            "-1.25", "6.0", "-6.0", "1.3", "-1.3", "2.8", "-2.8", "1.38", "-1.38", "0.8", "-0.8",
            "1.44", "-1.44", "3.0", "-3.0", "1.5", "-1.5", "0.2", "-0.2", "1.56", "-1.56", "3.2",
            "-3.2",
        ],
        &[
            "0x1.0#1", "-0x1.0#1", "0x2.0#1", "-0x2.0#1", "0x1.0#2", "-0x1.0#2", "0x0.8#1",
            "-0x0.8#1", "0x1.8#2", "-0x1.8#2", "0x2.0#2", "-0x2.0#2", "0x1.0#3", "-0x1.0#3",
            "0x4.0#1", "-0x4.0#1", "0x1.4#3", "-0x1.4#3", "0x3.0#2", "-0x3.0#2", "0x1.8#3",
            "-0x1.8#3", "0x0.8#2", "-0x0.8#2", "0x1.c#3", "-0x1.c#3", "0x2.0#3", "-0x2.0#3",
            "0x1.0#4", "-0x1.0#4", "0x0.4#1", "-0x0.4#1", "0x1.2#4", "-0x1.2#4", "0x2.8#3",
            "-0x2.8#3", "0x1.4#4", "-0x1.4#4", "0x0.c#2", "-0x0.c#2", "0x1.6#4", "-0x1.6#4",
            "0x3.0#3", "-0x3.0#3", "0x1.8#4", "-0x1.8#4", "0x4.0#2", "-0x4.0#2", "0x1.a#4",
            "-0x1.a#4", "0x3.8#3", "-0x3.8#3", "0x1.c#4", "-0x1.c#4", "0x0.8#3", "-0x0.8#3",
            "0x1.e#4", "-0x1.e#4", "0x2.0#4", "-0x2.0#4", "0x1.0#5", "-0x1.0#5", "0x8.0#1",
            "-0x8.0#1", "0x1.1#5", "-0x1.1#5", "0x2.4#4", "-0x2.4#4", "0x1.2#5", "-0x1.2#5",
            "0x0.a#3", "-0x0.a#3", "0x1.3#5", "-0x1.3#5", "0x2.8#4", "-0x2.8#4", "0x1.4#5",
            "-0x1.4#5", "0x6.0#2", "-0x6.0#2", "0x1.5#5", "-0x1.5#5", "0x2.c#4", "-0x2.c#4",
            "0x1.6#5", "-0x1.6#5", "0x0.c#3", "-0x0.c#3", "0x1.7#5", "-0x1.7#5", "0x3.0#4",
            "-0x3.0#4", "0x1.8#5", "-0x1.8#5", "0x0.4#2", "-0x0.4#2", "0x1.9#5", "-0x1.9#5",
            "0x3.4#4", "-0x3.4#4",
        ],
    );
}
