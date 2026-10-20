// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_matrix_unsigned_triple_gen_var_2;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_mul_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_mul_matrix);
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_mul_matrix_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_mul_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_mul_matrix_algorithms
    );
}

fn demo_natural_matrix_mod_power_of_2_mul_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, pow) in natural_matrix_natural_matrix_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_power_of_2_mul({b}, {pow}) = {}",
            (&a).mod_power_of_2_mul(&b, pow)
        );
    }
}

fn demo_natural_matrix_mod_power_of_2_mul_matrix_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, pow) in natural_matrix_natural_matrix_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_mul_assign(&b, pow);
        println!("a := {a_old}; a.mod_power_of_2_mul_assign({b}, {pow}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_power_of_2_mul_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_mul(NaturalMatrix, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_matrix_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_power_of_2_mul(NaturalMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_mul(b, pow));
                },
            ),
            (
                "NaturalMatrix.mod_power_of_2_mul(&NaturalMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_mul(&b, pow));
                },
            ),
            (
                "(&NaturalMatrix).mod_power_of_2_mul(NaturalMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_mul(b, pow));
                },
            ),
            (
                "(&NaturalMatrix).mod_power_of_2_mul(&NaturalMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_mul(&b, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_power_of_2_mul_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_mul(NaturalMatrix, u64)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_matrix_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, pow)| {
                no_out!((&a).mod_power_of_2_mul(&b, pow));
            }),
            ("naive", &mut |(a, b, pow)| {
                no_out!(natural_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow));
            }),
        ],
    );
}
