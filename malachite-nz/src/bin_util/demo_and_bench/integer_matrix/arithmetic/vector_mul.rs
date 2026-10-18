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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_integer_vector_pair_gen_var_2;
use malachite_nz::test_util::integer_matrix::arithmetic::mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_vector_mul);
    register_bench!(
        runner,
        benchmark_integer_matrix_vector_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_matrix_vector_mul_algorithms);
}

fn demo_integer_matrix_vector_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v) in integer_matrix_integer_vector_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{v} * {a} = {}", &v * &a);
    }
}

fn benchmark_integer_matrix_vector_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector * IntegerMatrix",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_integer_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerVector * IntegerMatrix", &mut |(a, v)| {
                no_out!(v * a);
            }),
            ("IntegerVector * &IntegerMatrix", &mut |(a, v)| {
                no_out!(v * &a);
            }),
            ("&IntegerVector * IntegerMatrix", &mut |(a, v)| {
                no_out!(&v * a);
            }),
            ("&IntegerVector * &IntegerMatrix", &mut |(a, v)| {
                no_out!(&v * &a);
            }),
        ],
    );
}

fn benchmark_integer_matrix_vector_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector * IntegerMatrix",
        BenchmarkType::Algorithms,
        integer_matrix_integer_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, v)| {
                no_out!(&v * &a);
            }),
            ("naive", &mut |(a, v)| {
                no_out!(integer_vector_mul_integer_matrix_naive(&v, &a));
            }),
        ],
    );
}
