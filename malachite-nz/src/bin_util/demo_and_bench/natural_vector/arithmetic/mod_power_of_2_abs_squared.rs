// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_abs_squared);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_abs_squared_ref);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_abs_squared_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.mod_power_of_2_abs_squared({pow}) = {}",
            v.mod_power_of_2_abs_squared(pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_abs_squared_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_abs_squared({pow}) = {}",
            (&v).mod_power_of_2_abs_squared(pow)
        );
    }
}

fn benchmark_natural_vector_mod_power_of_2_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_abs_squared(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_power_of_2_abs_squared(u64)",
                &mut |(v, pow)| {
                    no_out!(v.mod_power_of_2_abs_squared(pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_abs_squared(u64)",
                &mut |(v, pow)| {
                    no_out!((&v).mod_power_of_2_abs_squared(pow));
                },
            ),
        ],
    );
}
