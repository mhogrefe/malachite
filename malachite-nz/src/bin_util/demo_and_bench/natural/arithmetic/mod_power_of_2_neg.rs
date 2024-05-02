// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModNeg, ModPowerOf2, ModPowerOf2Neg, ModPowerOf2NegAssign, PowerOf2,
};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_11;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_neg_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_neg);
    register_demo!(runner, demo_natural_mod_power_of_2_neg_ref);

    register_bench!(runner, benchmark_natural_mod_power_of_2_neg_assign);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_neg_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_neg_algorithms);
}

fn demo_natural_mod_power_of_2_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_neg_assign(pow);
        println!("x := {n_old}; x.mod_power_of_2_neg_assign({pow}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("-{} ≡ {} mod 2^{}", n_old, n.mod_power_of_2_neg(pow), pow);
    }
}

fn demo_natural_mod_power_of_2_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "-(&{}) ≡ {} mod 2^{}",
            n_old,
            n.mod_power_of_2_neg(pow),
            pow
        );
    }
}

fn benchmark_natural_mod_power_of_2_neg_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_neg_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Natural.mod_power_of_2_neg_assign(u64)", &mut |(
            mut n,
            pow,
        )| {
            n.mod_power_of_2_neg_assign(pow)
        })],
    );
}

fn benchmark_natural_mod_power_of_2_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_neg(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_neg(pow))
            }),
            ("(&Natural).mod_power_of_2_neg(u64)", &mut |(n, pow)| {
                no_out!((&n).mod_power_of_2_neg(pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_neg(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_neg(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_neg(pow))
            }),
            ("(-Natural).mod_power_of_2(u64)", &mut |(n, pow)| {
                no_out!((-n).mod_power_of_2(pow))
            }),
            (
                "Natural.mod_neg(Natural::power_of_2(u64))",
                &mut |(n, pow)| no_out!(n.mod_neg(Natural::power_of_2(pow))),
            ),
        ],
    );
}
