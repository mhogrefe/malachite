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
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_rational_vector_pair_gen_var_2;
use malachite_q::test_util::rational_matrix::arithmetic::mul_vector::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_vector_mul);
    register_bench!(
        runner,
        benchmark_rational_matrix_vector_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_matrix_vector_mul_algorithms);
}

fn demo_rational_matrix_vector_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, v) in rational_matrix_rational_vector_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{v} * {a} = {}", &v * &a);
    }
}

fn benchmark_rational_matrix_vector_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector * RationalMatrix",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_rational_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalVector * RationalMatrix", &mut |(a, v)| {
                no_out!(v * a);
            }),
            ("RationalVector * &RationalMatrix", &mut |(a, v)| {
                no_out!(v * &a);
            }),
            ("&RationalVector * RationalMatrix", &mut |(a, v)| {
                no_out!(&v * a);
            }),
            ("&RationalVector * &RationalMatrix", &mut |(a, v)| {
                no_out!(&v * &a);
            }),
        ],
    );
}

fn benchmark_rational_matrix_vector_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector * RationalMatrix",
        BenchmarkType::Algorithms,
        rational_matrix_rational_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, v)| {
                no_out!(&v * &a);
            }),
            ("naive", &mut |(a, v)| {
                no_out!(rational_vector_mul_rational_matrix_naive(&v, &a));
            }),
        ],
    );
}
