// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::random::random_bools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{uniform_bool_assertions, MomentStats};

#[test]
fn test_random_bools() {
    uniform_bool_assertions(
        random_bools(EXAMPLE_SEED),
        false,
        true,
        &[
            true, false, false, false, true, true, true, false, true, true, true, true, false,
            true, true, true, true, false, true, false,
        ],
        &[(true, 500473), (false, 499527)],
        (false, Some(true)),
        (true, None),
        MomentStats {
            mean: NiceFloat(0.5),
            standard_deviation: NiceFloat(0.5),
            skewness: NiceFloat(0.0),
            excess_kurtosis: NiceFloat(-1.9999999999999998),
        },
        MomentStats {
            mean: NiceFloat(0.5004730000000077),
            standard_deviation: NiceFloat(0.5000000262710417),
            skewness: NiceFloat(-0.0018920008465908307),
            excess_kurtosis: NiceFloat(-1.999996420332894),
        },
    );
}
