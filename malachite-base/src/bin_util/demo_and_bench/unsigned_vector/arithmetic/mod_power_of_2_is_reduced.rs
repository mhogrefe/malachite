// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_is_reduced);
    register_bench!(runner, benchmark_unsigned_vector_mod_power_of_2_is_reduced);
}

fn demo_unsigned_vector_mod_power_of_2_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in unsigned_vector_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({v}).mod_power_of_2_is_reduced({pow}) = {}",
            v.mod_power_of_2_is_reduced(pow)
        );
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_power_of_2_is_reduced(u64)",
        BenchmarkType::Single,
        unsigned_vector_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |(v, pow)| {
            no_out!(v.mod_power_of_2_is_reduced(pow));
        })],
    );
}
