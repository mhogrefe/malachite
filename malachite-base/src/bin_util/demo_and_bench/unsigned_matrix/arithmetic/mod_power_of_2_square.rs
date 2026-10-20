// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_7;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_square);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_square_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_square_assign);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_square_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_square({pow}) = {}",
            a.mod_power_of_2_square(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, pow) in unsigned_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_square({pow}) = {}",
            (&a).mod_power_of_2_square(pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_square_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, pow) in unsigned_matrix_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_square_assign(pow);
        println!("a := {a_old}; a.mod_power_of_2_square_assign({pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_power_of_2_square(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_square(u64)",
                &mut |(a, pow)| {
                    no_out!(a.mod_power_of_2_square(pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_square(u64)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_square(pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_square_assign(u64)",
                &mut |(mut a, pow)| {
                    a.mod_power_of_2_square_assign(pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix<u64>).mod_power_of_2_square(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, pow)| {
                no_out!((&a).mod_power_of_2_square(pow));
            }),
            ("using mod_power_of_2_mul", &mut |(a, pow)| {
                no_out!((&a).mod_power_of_2_mul(&a, pow));
            }),
            ("naive", &mut |(a, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_square_naive(&a, pow));
            }),
        ],
    );
}
