// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModInverse, ModPowerOf2Inverse, PowerOf2};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_14;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_inverse);
    register_demo!(runner, demo_natural_mod_power_of_2_inverse_ref);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_inverse_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_inverse_algorithms);
}

fn demo_natural_mod_power_of_2_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        if let Some(inverse) = n.mod_power_of_2_inverse(pow) {
            println!("{n_old}⁻¹ ≡ {inverse} mod 2^{pow}");
        } else {
            println!("{n_old} is not invertible mod 2^{pow}");
        }
    }
}

fn demo_natural_mod_power_of_2_inverse_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        if let Some(inverse) = (&n).mod_power_of_2_inverse(pow) {
            println!("{n}⁻¹ ≡ {inverse} mod 2^{pow}");
        } else {
            println!("{n} is not invertible mod 2^{pow}");
        }
    }
}

fn benchmark_natural_mod_power_of_2_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_inverse(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_inverse(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_inverse(pow))
            }),
            ("(&Natural).mod_power_of_2_inverse(u64)", &mut |(n, pow)| {
                no_out!((&n).mod_power_of_2_inverse(pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_inverse_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_inverse(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("default", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_inverse(pow))
            }),
            ("simple", &mut |(n, pow)| {
                no_out!(n.mod_inverse(Natural::power_of_2(pow)))
            }),
        ],
    );
}
