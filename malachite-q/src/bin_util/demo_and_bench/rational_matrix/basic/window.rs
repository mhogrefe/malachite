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
use malachite_q::test_util::bench::bucketers::quintuple_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_window);
    register_demo!(runner, demo_rational_matrix_into_window);
    register_bench!(runner, benchmark_rational_matrix_window_evaluation_strategy);
}

fn demo_rational_matrix_window(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, r1, r2, c1, c2) in
        rational_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "{m}.window({r1}..{r2}, {c1}..{c2}) = {}",
            m.window(r1..r2, c1..c2)
        );
    }
}

fn demo_rational_matrix_into_window(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, r1, r2, c1, c2) in
        rational_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let m_old = m.clone();
        println!(
            "{m_old}.into_window({r1}..{r2}, {c1}..{c2}) = {}",
            m.into_window(r1..r2, c1..c2)
        );
    }
}

fn benchmark_rational_matrix_window_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.window(Range<u64>, Range<u64>)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_rational_matrix_bit_bucketer("m"),
        &mut [
            (
                "RationalMatrix.window(Range<u64>, Range<u64>)",
                &mut |(m, r1, r2, c1, c2)| {
                    no_out!(m.window(r1..r2, c1..c2));
                },
            ),
            (
                "RationalMatrix.into_window(Range<u64>, Range<u64>)",
                &mut |(m, r1, r2, c1, c2)| {
                    no_out!(m.into_window(r1..r2, c1..c2));
                },
            ),
        ],
    );
}
