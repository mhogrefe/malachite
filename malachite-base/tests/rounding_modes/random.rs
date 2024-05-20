// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::stats::common_values_map::common_values_map;

#[test]
fn test_random_rounding_modes() {
    let xs = random_rounding_modes(EXAMPLE_SEED);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map(1000000, 10, xs.clone());
    assert_eq!(
        (values.as_slice(), common_values.as_slice()),
        (
            &[
                Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact, Nearest, Down,
                Exact, Down, Floor, Exact, Floor, Down, Nearest, Down
            ][..],
            &[
                (Ceiling, 167408),
                (Down, 167104),
                (Nearest, 166935),
                (Exact, 166549),
                (Floor, 166068),
                (Up, 165936)
            ][..]
        )
    );
}
