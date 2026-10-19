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
use malachite_q::test_util::bench::bucketers::pair_rational_matrix_max_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_concat_horizontal);
    register_demo!(runner, demo_rational_matrix_into_concat_horizontal);
    register_bench!(
        runner,
        benchmark_rational_matrix_concat_horizontal_evaluation_strategy
    );
}

fn demo_rational_matrix_concat_horizontal(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        println!("{a}.concat_horizontal(&{b}) = {}", a.concat_horizontal(&b));
    }
}

fn demo_rational_matrix_into_concat_horizontal(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!(
            "{a_old}.into_concat_horizontal({b_old}) = {}",
            a.into_concat_horizontal(b)
        );
    }
}

fn benchmark_rational_matrix_concat_horizontal_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.concat_horizontal(&RationalMatrix)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_matrix_max_bit_bucketer("a", "b"),
        &mut [
            (
                "RationalMatrix.concat_horizontal(&RationalMatrix)",
                &mut |(a, b)| {
                    no_out!(a.concat_horizontal(&b));
                },
            ),
            (
                "RationalMatrix.into_concat_horizontal(RationalMatrix)",
                &mut |(a, b)| {
                    no_out!(a.into_concat_horizontal(b));
                },
            ),
        ],
    );
}
