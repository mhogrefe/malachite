// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_natural_vector_pair_gen_var_1;
use malachite_nz::test_util::natural_matrix::arithmetic::mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mul_vector);
    register_bench!(
        runner,
        benchmark_natural_matrix_mul_vector_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mul_vector_algorithms);
}

fn demo_natural_matrix_mul_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v) in natural_matrix_natural_vector_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("{a} * {v} = {}", &a * &v);
    }
}

fn benchmark_natural_matrix_mul_vector_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix * NaturalVector",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix * NaturalVector", &mut |(a, v)| {
                no_out!(a * v);
            }),
            ("NaturalMatrix * &NaturalVector", &mut |(a, v)| {
                no_out!(a * &v);
            }),
            ("&NaturalMatrix * NaturalVector", &mut |(a, v)| {
                no_out!(&a * v);
            }),
            ("&NaturalMatrix * &NaturalVector", &mut |(a, v)| {
                no_out!(&a * &v);
            }),
        ],
    );
}

fn benchmark_natural_matrix_mul_vector_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix * NaturalVector",
        BenchmarkType::Algorithms,
        natural_matrix_natural_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, v)| {
                no_out!(&a * &v);
            }),
            ("naive", &mut |(a, v)| {
                no_out!(natural_matrix_mul_vector_naive(&a, &v));
            }),
        ],
    );
}
