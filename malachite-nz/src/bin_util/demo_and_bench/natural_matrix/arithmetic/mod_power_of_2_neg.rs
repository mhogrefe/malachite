// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Neg, ModPowerOf2NegAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_neg);
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_neg_ref);
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_neg_assign);

    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_neg_evaluation_strategy
    );
}

fn demo_natural_matrix_mod_power_of_2_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "({a_old}).mod_power_of_2_neg({pow}) = {}",
            a.mod_power_of_2_neg(pow)
        );
    }
}

fn demo_natural_matrix_mod_power_of_2_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({a})).mod_power_of_2_neg({pow}) = {}",
            (&a).mod_power_of_2_neg(pow)
        );
    }
}

fn demo_natural_matrix_mod_power_of_2_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, pow) in natural_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_neg_assign(pow);
        println!("a := {a_old}; a.mod_power_of_2_neg_assign({pow}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_power_of_2_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_power_of_2_neg(u64)", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2_neg(pow));
            }),
            (
                "(&NaturalMatrix).mod_power_of_2_neg(u64)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_neg(pow));
                },
            ),
            (
                "NaturalMatrix.mod_power_of_2_neg_assign(u64)",
                &mut |(mut a, pow)| a.mod_power_of_2_neg_assign(pow),
            ),
        ],
    );
}
