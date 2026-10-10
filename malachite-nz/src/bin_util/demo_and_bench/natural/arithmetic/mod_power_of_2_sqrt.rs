// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2Sqrt;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_11, natural_unsigned_pair_gen_var_15,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_sqrt::mod_power_of_2_sqrt_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_sqrt);
    register_demo!(runner, demo_natural_mod_power_of_2_sqrt_ref);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_sqrt_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_sqrt_algorithms);
}

fn demo_natural_mod_power_of_2_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_2_sqrt({}) = {:?}",
            n_old,
            pow,
            n.mod_power_of_2_sqrt(pow)
        );
    }
}

fn demo_natural_mod_power_of_2_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_power_of_2_sqrt({}) = {:?}",
            n,
            pow,
            (&n).mod_power_of_2_sqrt(pow)
        );
    }
}

fn benchmark_natural_mod_power_of_2_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_sqrt(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_sqrt(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_sqrt(pow));
            }),
            ("(&Natural).mod_power_of_2_sqrt(u64)", &mut |(n, pow)| {
                no_out!((&n).mod_power_of_2_sqrt(pow));
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sqrt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_sqrt(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("default", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_sqrt(pow));
            }),
            ("naive", &mut |(n, pow)| {
                no_out!(mod_power_of_2_sqrt_naive(&n, pow));
            }),
        ],
    );
}
