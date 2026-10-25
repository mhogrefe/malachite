// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Solve};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_solve);
}

// For each matrix A, the systems with right-hand sides its first column, which are consistent, and
// the first unit vector, which may not be.
fn demo_unsigned_matrix_mod_power_of_2_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.height() != 0 && a.width() != 0)
        .take(limit)
    {
        let h = usize::try_from(a.height()).unwrap();
        let w = usize::try_from(a.width()).unwrap();
        let first_column = UnsignedVector {
            elements: (0..h).map(|i| a.elements[i * w]).collect(),
        };
        let mut unit = vec![0u64; h];
        unit[0] = 1u64.mod_power_of_2(pow);
        let unit = UnsignedVector { elements: unit };
        for b in [first_column, unit] {
            println!(
                "(&{a}).mod_power_of_2_solve(&{b}, {pow}) = {}",
                (&a).mod_power_of_2_solve(&b, pow).to_debug_string()
            );
        }
    }
}
