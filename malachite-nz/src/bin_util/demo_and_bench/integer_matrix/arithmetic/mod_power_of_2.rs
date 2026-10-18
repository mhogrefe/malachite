// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, RemPowerOf2, RemPowerOf2Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_unsigned_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_mod_power_of_2);
    register_demo!(runner, demo_integer_matrix_mod_power_of_2_ref);
    register_demo!(runner, demo_integer_matrix_rem_power_of_2);
    register_demo!(runner, demo_integer_matrix_rem_power_of_2_ref);
    register_demo!(runner, demo_integer_matrix_rem_power_of_2_assign);

    register_bench!(
        runner,
        benchmark_integer_matrix_mod_power_of_2_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_rem_power_of_2_evaluation_strategy
    );
}

fn demo_integer_matrix_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in integer_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_power_of_2({pow}) = {}", a.mod_power_of_2(pow));
    }
}

fn demo_integer_matrix_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in integer_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2({pow}) = {}",
            (&a).mod_power_of_2(pow)
        );
    }
}

fn demo_integer_matrix_rem_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in integer_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.rem_power_of_2({pow}) = {}", a.rem_power_of_2(pow));
    }
}

fn demo_integer_matrix_rem_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in integer_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).rem_power_of_2({pow}) = {}",
            (&a).rem_power_of_2(pow)
        );
    }
}

fn demo_integer_matrix_rem_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, pow) in integer_matrix_unsigned_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.rem_power_of_2_assign(pow);
        println!("a := {a_old}; a.rem_power_of_2_assign({pow}); a = {a}");
    }
}

fn benchmark_integer_matrix_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.mod_power_of_2(u64)", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2(pow));
            }),
            ("(&IntegerMatrix).mod_power_of_2(u64)", &mut |(a, pow)| {
                no_out!((&a).mod_power_of_2(pow));
            }),
        ],
    );
}

fn benchmark_integer_matrix_rem_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.rem_power_of_2(u64)", &mut |(a, pow)| {
                no_out!(a.rem_power_of_2(pow));
            }),
            ("(&IntegerMatrix).rem_power_of_2(u64)", &mut |(a, pow)| {
                no_out!((&a).rem_power_of_2(pow));
            }),
            (
                "IntegerMatrix.rem_power_of_2_assign(u64)",
                &mut |(mut a, pow)| {
                    a.rem_power_of_2_assign(pow);
                },
            ),
        ],
    );
}
