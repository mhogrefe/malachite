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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_integer_matrix_pair_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::mul_integer_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_mul_integer_matrix);
    register_demo!(runner, demo_rational_matrix_mul_assign_integer_matrix);
    register_bench!(
        runner,
        benchmark_rational_matrix_mul_integer_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_mul_integer_matrix_algorithms
    );
}

fn demo_rational_matrix_mul_integer_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_integer_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("{a} * {b} = {}", &a * &b);
    }
}

fn demo_rational_matrix_mul_assign_integer_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in rational_matrix_integer_matrix_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a *= &b;
        println!("a := {a_old}; a *= {b}; a = {a}");
    }
}

fn benchmark_rational_matrix_mul_integer_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix * IntegerMatrix",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_integer_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix * IntegerMatrix", &mut |(a, b)| {
                no_out!(a * b);
            }),
            ("RationalMatrix * &IntegerMatrix", &mut |(a, b)| {
                no_out!(a * &b);
            }),
            ("&RationalMatrix * IntegerMatrix", &mut |(a, b)| {
                no_out!(&a * b);
            }),
            ("&RationalMatrix * &IntegerMatrix", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("RationalMatrix *= IntegerMatrix", &mut |(mut a, b)| {
                a *= b;
            }),
            ("RationalMatrix *= &IntegerMatrix", &mut |(mut a, b)| {
                a *= &b;
            }),
        ],
    );
}

fn benchmark_rational_matrix_mul_integer_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix * IntegerMatrix",
        BenchmarkType::Algorithms,
        rational_matrix_integer_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(&a * &b);
            }),
            ("converting to RationalMatrix", &mut |(a, b)| {
                no_out!(&a * &RationalMatrix::from(b));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(rational_matrix_mul_integer_matrix_naive(&a, &b));
            }),
        ],
    );
}
