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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::integer_matrix_rational_matrix_pair_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::integer_matrix_mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_integer_matrix_mul);
    register_bench!(
        runner,
        benchmark_rational_matrix_integer_matrix_mul_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_integer_matrix_mul_algorithms
    );
}

fn demo_rational_matrix_integer_matrix_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_rational_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("{a} * {b} = {}", &a * &b);
    }
}

fn benchmark_rational_matrix_integer_matrix_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix * RationalMatrix",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_rational_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix * RationalMatrix", &mut |(a, b)| {
                no_out!(a * b);
            }),
            ("IntegerMatrix * &RationalMatrix", &mut |(a, b)| {
                no_out!(a * &b);
            }),
            ("&IntegerMatrix * RationalMatrix", &mut |(a, b)| {
                no_out!(&a * b);
            }),
            ("&IntegerMatrix * &RationalMatrix", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
        ],
    );
}

fn benchmark_rational_matrix_integer_matrix_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix * RationalMatrix",
        BenchmarkType::Algorithms,
        integer_matrix_rational_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("converting to RationalMatrix", &mut |(a, b)| {
                no_out!(&RationalMatrix::from(a) * &b);
            }),
            ("naive", &mut |(a, b)| {
                no_out!(integer_matrix_mul_rational_matrix_naive(&a, &b));
            }),
        ],
    );
}
