// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bin_util::demo_and_bench::integer_matrix::arithmetic::solve::format_solution;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::can_solve_multi_mod::can_solve_multi_mod;
use malachite_nz::integer_matrix::arithmetic::solve_dixon::{solve_dixon, solve_dixon_den};
use malachite_nz::integer_matrix::arithmetic::solve_multi_mod::solve_multi_mod;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_height_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_solve_dixon_den);
    register_demo!(runner, demo_integer_matrix_solve_dixon_den_low_rank);
    register_bench!(runner, benchmark_integer_matrix_solve_dixon_algorithms);
}

fn demo_integer_matrix_solve_dixon_den(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "solve_dixon_den({a}, {b}) = {}",
            format_solution(solve_dixon_den(&a, &b))
        );
    }
}

// Systems whose matrix has rank at most the width of the right-hand side, when that is smaller than
// its dimension, so that many are singular.
fn demo_integer_matrix_solve_dixon_den_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!(
            "solve_dixon_den({a}, {b}) = {}",
            format_solution(solve_dixon_den(&a, &b))
        );
    }
}

fn benchmark_integer_matrix_solve_dixon_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "solve_dixon(&IntegerMatrix, &IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_height_bucketer("a"),
        &mut [
            ("Dixon", &mut |(a, b)| {
                no_out!(solve_dixon(&a, &b));
            }),
            ("multimodular", &mut |(a, b)| {
                no_out!(solve_multi_mod(&a, &b));
            }),
            ("multimodular, any shape", &mut |(a, b)| {
                no_out!(can_solve_multi_mod(&a, &b));
            }),
        ],
    );
}
