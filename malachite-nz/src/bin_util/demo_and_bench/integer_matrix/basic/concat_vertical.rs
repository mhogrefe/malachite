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
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_concat_vertical);
    register_demo!(runner, demo_integer_matrix_into_concat_vertical);
    register_bench!(
        runner,
        benchmark_integer_matrix_concat_vertical_evaluation_strategy
    );
}

fn demo_integer_matrix_concat_vertical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_2().get(gm, config).take(limit) {
        println!("{a}.concat_vertical(&{b}) = {}", a.concat_vertical(&b));
    }
}

fn demo_integer_matrix_into_concat_vertical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_2().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!(
            "{a_old}.into_concat_vertical({b_old}) = {}",
            a.into_concat_vertical(b)
        );
    }
}

fn benchmark_integer_matrix_concat_vertical_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.concat_vertical(&IntegerMatrix)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            (
                "IntegerMatrix.concat_vertical(&IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!(a.concat_vertical(&b));
                },
            ),
            (
                "IntegerMatrix.into_concat_vertical(IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!(a.into_concat_vertical(b));
                },
            ),
        ],
    );
}
