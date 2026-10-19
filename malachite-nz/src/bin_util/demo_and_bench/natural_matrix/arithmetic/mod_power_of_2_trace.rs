// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2Trace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_6;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_to_mod_power_of_2_trace);
    register_demo!(runner, demo_natural_matrix_into_mod_power_of_2_trace);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_trace_evaluation_strategy
    );
}

fn demo_natural_matrix_to_mod_power_of_2_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.to_mod_power_of_2_trace({pow}) = {}",
            a.to_mod_power_of_2_trace(pow)
        );
    }
}

fn demo_natural_matrix_into_mod_power_of_2_trace(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.into_mod_power_of_2_trace({pow}) = {}",
            a.into_mod_power_of_2_trace(pow)
        );
    }
}

fn benchmark_natural_matrix_mod_power_of_2_trace_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.to_mod_power_of_2_trace(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.to_mod_power_of_2_trace(u64)",
                &mut |(a, pow)| {
                    no_out!(a.to_mod_power_of_2_trace(pow));
                },
            ),
            (
                "NaturalMatrix.into_mod_power_of_2_trace(u64)",
                &mut |(a, pow)| {
                    no_out!(a.into_mod_power_of_2_trace(pow));
                },
            ),
        ],
    );
}
