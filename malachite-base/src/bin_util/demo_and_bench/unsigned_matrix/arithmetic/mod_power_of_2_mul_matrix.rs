// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_power_of_2_mul_matrix);
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_mul_matrix_assign
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_mul_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_mul_matrix_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_mul_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, pow) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_power_of_2_mul({b}, {pow}) = {}",
            (&a).mod_power_of_2_mul(&b, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_mul_matrix_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, pow) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_mul_assign(&b, pow);
        println!("a := {a_old}; a.mod_power_of_2_mul_assign({b}, {pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_mul_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_power_of_2_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_mul(UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_mul(b, pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_mul(&UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_mul(&b, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_mul(UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_mul(b, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_mul(&UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_mul(&b, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_mul_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_power_of_2_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, pow)| {
                no_out!((&a).mod_power_of_2_mul(&b, pow));
            }),
            ("classical", &mut |(a, b, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_mul_matrix_classical(
                    &a, &b, pow
                ));
            }),
            ("Strassen", &mut |(a, b, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_mul_matrix_strassen(
                    &a, &b, pow
                ));
            }),
            ("naive", &mut |(a, b, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow));
            }),
        ],
    );
}
