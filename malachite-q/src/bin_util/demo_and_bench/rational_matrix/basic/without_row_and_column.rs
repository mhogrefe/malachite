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
use malachite_q::test_util::bench::bucketers::triple_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_unsigned_unsigned_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_without_row_and_column);
    register_demo!(runner, demo_rational_matrix_into_without_row_and_column);
    register_bench!(
        runner,
        benchmark_rational_matrix_without_row_and_column_evaluation_strategy
    );
}

fn demo_rational_matrix_without_row_and_column(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i, j) in rational_matrix_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (i, j) = (u64::try_from(i).unwrap(), u64::try_from(j).unwrap());
        println!(
            "{m}.without_row_and_column({i}, {j}) = {}",
            m.without_row_and_column(i, j)
        );
    }
}

fn demo_rational_matrix_into_without_row_and_column(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i, j) in rational_matrix_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (i, j) = (u64::try_from(i).unwrap(), u64::try_from(j).unwrap());
        let m_old = m.clone();
        println!(
            "{m_old}.into_without_row_and_column({i}, {j}) = {}",
            m.into_without_row_and_column(i, j)
        );
    }
}

fn benchmark_rational_matrix_without_row_and_column_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.without_row_and_column(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_matrix_bit_bucketer("m"),
        &mut [
            (
                "RationalMatrix.without_row_and_column(u64, u64)",
                &mut |(m, i, j)| {
                    no_out!(m.without_row_and_column(
                        u64::try_from(i).unwrap(),
                        u64::try_from(j).unwrap()
                    ));
                },
            ),
            (
                "RationalMatrix.into_without_row_and_column(u64, u64)",
                &mut |(m, i, j)| {
                    no_out!(m.into_without_row_and_column(
                        u64::try_from(i).unwrap(),
                        u64::try_from(j).unwrap()
                    ));
                },
            ),
        ],
    );
}
