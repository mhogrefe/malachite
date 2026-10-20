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
use malachite_nz::integer_matrix::arithmetic::square::bodrato::matrix_square_bodrato;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::square::{
    integer_matrix_square_naive, integer_matrix_square_with,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_square);
    register_demo!(runner, demo_integer_matrix_square_ref);
    register_demo!(runner, demo_integer_matrix_square_assign);

    register_bench!(runner, benchmark_integer_matrix_square_evaluation_strategy);
    register_bench!(runner, benchmark_integer_matrix_square_algorithms);
}

fn demo_integer_matrix_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.square() = {}", a.square());
    }
}

fn demo_integer_matrix_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).square() = {}", (&a).square());
    }
}

fn demo_integer_matrix_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.square_assign();
        println!("a := {a_old}; a.square_assign(); a = {a}");
    }
}

fn benchmark_integer_matrix_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.square()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.square()", &mut |a| {
                no_out!(a.square());
            }),
            ("(&IntegerMatrix).square()", &mut |a| {
                no_out!((&a).square());
            }),
            ("IntegerMatrix.square_assign()", &mut |mut a| {
                a.square_assign();
            }),
        ],
    );
}

fn benchmark_integer_matrix_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).square()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!((&a).square());
            }),
            ("using *", &mut |a| {
                no_out!(&a * &a);
            }),
            ("Bodrato", &mut |a| {
                no_out!(integer_matrix_square_with(&a, matrix_square_bodrato));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_square_naive(&a));
            }),
        ],
    );
}
