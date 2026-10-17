// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_is_reduced);
    register_bench!(runner, benchmark_natural_matrix_mod_power_of_2_is_reduced);
}

fn demo_natural_matrix_mod_power_of_2_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, pow) in natural_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({m}).mod_power_of_2_is_reduced({pow}) = {}",
            m.mod_power_of_2_is_reduced(pow)
        );
    }
}

fn benchmark_natural_matrix_mod_power_of_2_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_is_reduced(u64)",
        BenchmarkType::Single,
        natural_matrix_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(m, pow)| {
            no_out!(m.mod_power_of_2_is_reduced(pow));
        })],
    );
}
