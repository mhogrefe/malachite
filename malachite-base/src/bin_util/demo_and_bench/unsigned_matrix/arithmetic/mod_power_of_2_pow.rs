// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_8;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_pow);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_pow_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_pow_assign);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_pow_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_pow_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_pow({exp}, {pow}) = {}",
            a.mod_power_of_2_pow(exp, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_pow({exp}, {pow}) = {}",
            (&a).mod_power_of_2_pow(exp, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, exp, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_pow_assign(exp, pow);
        println!("a := {a_old}; a.mod_power_of_2_pow_assign({exp}, {pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_power_of_2_pow(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_pow(u64, u64)",
                &mut |(a, exp, pow)| {
                    no_out!(a.mod_power_of_2_pow(exp, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_pow(u64, u64)",
                &mut |(a, exp, pow)| {
                    no_out!((&a).mod_power_of_2_pow(exp, pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_pow_assign(u64, u64)",
                &mut |(mut a, exp, pow)| {
                    a.mod_power_of_2_pow_assign(exp, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix<u64>).mod_power_of_2_pow(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, exp, pow)| {
                no_out!((&a).mod_power_of_2_pow(exp, pow));
            }),
            ("naive", &mut |(a, exp, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_pow_naive(&a, exp, pow));
            }),
        ],
    );
}
