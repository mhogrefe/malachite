// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_vector_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_power_of_2_mul_vector);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_mul_vector_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_mul_vector_algorithms
    );
}

fn demo_natural_matrix_mod_power_of_2_mul_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v, pow) in natural_matrix_natural_vector_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_power_of_2_mul({v}, {pow}) = {}",
            (&a).mod_power_of_2_mul(&v, pow)
        );
    }
}

fn benchmark_natural_matrix_mod_power_of_2_mul_vector_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_mul(NaturalVector, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_vector_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_power_of_2_mul(NaturalVector, u64)",
                &mut |(a, v, pow)| {
                    no_out!(a.mod_power_of_2_mul(v, pow));
                },
            ),
            (
                "NaturalMatrix.mod_power_of_2_mul(&NaturalVector, u64)",
                &mut |(a, v, pow)| {
                    no_out!(a.mod_power_of_2_mul(&v, pow));
                },
            ),
            (
                "(&NaturalMatrix).mod_power_of_2_mul(NaturalVector, u64)",
                &mut |(a, v, pow)| {
                    no_out!((&a).mod_power_of_2_mul(v, pow));
                },
            ),
            (
                "(&NaturalMatrix).mod_power_of_2_mul(&NaturalVector, u64)",
                &mut |(a, v, pow)| {
                    no_out!((&a).mod_power_of_2_mul(&v, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_power_of_2_mul_vector_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_mul(NaturalVector, u64)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_vector_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, v, pow)| {
                no_out!((&a).mod_power_of_2_mul(&v, pow));
            }),
            ("naive", &mut |(a, v, pow)| {
                no_out!(natural_matrix_mod_power_of_2_mul_vector_naive(&a, &v, pow));
            }),
        ],
    );
}
