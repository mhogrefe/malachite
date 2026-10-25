// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModDeterminant, ModPowerOf2Determinant};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_determinant);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_determinant_ref);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_determinant_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_determinant_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_determinant(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.height() == a.width())
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_determinant({pow}) = {}",
            a.mod_power_of_2_determinant(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_determinant_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.height() == a.width())
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_determinant({pow}) = {}",
            (&a).mod_power_of_2_determinant(pow)
        );
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_determinant_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_determinant(pow)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.height() == a.width()),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_determinant(pow)",
                &mut |(a, pow)| {
                    no_out!(a.mod_power_of_2_determinant(pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_determinant(pow)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_determinant(pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_determinant_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_determinant(pow)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, pow)| *pow < 64 && a.height() == a.width()),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2_determinant(pow));
            }),
            ("modulo 2^pow, general", &mut |(a, pow)| {
                no_out!(a.mod_determinant(1u64 << pow));
            }),
        ],
    );
}
