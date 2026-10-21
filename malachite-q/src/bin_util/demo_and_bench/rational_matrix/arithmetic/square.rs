// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_square);
    register_demo!(runner, demo_rational_matrix_square_ref);
    register_demo!(runner, demo_rational_matrix_square_assign);
    register_bench!(runner, benchmark_rational_matrix_square_evaluation_strategy);
    register_bench!(runner, benchmark_rational_matrix_square_algorithms);
}

fn demo_rational_matrix_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.square() = {}", a.square());
    }
}

fn demo_rational_matrix_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).square() = {}", (&a).square());
    }
}

fn demo_rational_matrix_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.square_assign();
        println!("a := {a_old}; a.square_assign(); a = {a}");
    }
}

fn benchmark_rational_matrix_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.square()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.square()", &mut |a| {
                no_out!(a.square());
            }),
            ("(&RationalMatrix).square()", &mut |a| {
                no_out!((&a).square());
            }),
            ("RationalMatrix.square_assign()", &mut |mut a| {
                a.square_assign();
            }),
        ],
    );
}

fn benchmark_rational_matrix_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).square()",
        BenchmarkType::Algorithms,
        rational_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!((&a).square());
            }),
            ("using *", &mut |a| {
                no_out!(&a * &a);
            }),
            ("matwise denominators", &mut |a| {
                no_out!(rational_matrix_square_matwise(&a));
            }),
            ("naive", &mut |a| {
                no_out!(rational_matrix_square_naive(&a));
            }),
        ],
    );
}
