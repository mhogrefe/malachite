// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_set_window);
    register_demo!(runner, demo_integer_matrix_set_window_owned);
    register_bench!(
        runner,
        benchmark_integer_matrix_set_window_evaluation_strategy
    );
}

fn demo_integer_matrix_set_window(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, w, i, j) in integer_matrix_integer_matrix_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        m.set_window(i, j, &w);
        println!("m := {m_old}; m.set_window({i}, {j}, &{w}); m = {m}");
    }
}

fn demo_integer_matrix_set_window_owned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, w, i, j) in integer_matrix_integer_matrix_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        let w_old = w.clone();
        m.set_window_owned(i, j, w);
        println!("m := {m_old}; m.set_window_owned({i}, {j}, {w_old}); m = {m}");
    }
}

fn benchmark_integer_matrix_set_window_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.set_window(u64, u64, &IntegerMatrix)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_integer_matrix_unsigned_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_integer_matrix_bit_bucketer("m"),
        &mut [
            (
                "IntegerMatrix.set_window(u64, u64, &IntegerMatrix)",
                &mut |(mut m, w, i, j)| {
                    m.set_window(i, j, &w);
                },
            ),
            (
                "IntegerMatrix.set_window_owned(u64, u64, IntegerMatrix)",
                &mut |(mut m, w, i, j)| {
                    m.set_window_owned(i, j, w);
                },
            ),
        ],
    );
}
