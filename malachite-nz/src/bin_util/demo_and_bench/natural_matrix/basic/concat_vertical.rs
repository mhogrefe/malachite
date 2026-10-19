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
use malachite_nz::test_util::bench::bucketers::pair_natural_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_concat_vertical);
    register_demo!(runner, demo_natural_matrix_into_concat_vertical);
    register_bench!(
        runner,
        benchmark_natural_matrix_concat_vertical_evaluation_strategy
    );
}

fn demo_natural_matrix_concat_vertical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_2().get(gm, config).take(limit) {
        println!("{a}.concat_vertical(&{b}) = {}", a.concat_vertical(&b));
    }
}

fn demo_natural_matrix_into_concat_vertical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_2().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!(
            "{a_old}.into_concat_vertical({b_old}) = {}",
            a.into_concat_vertical(b)
        );
    }
}

fn benchmark_natural_matrix_concat_vertical_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.concat_vertical(&NaturalMatrix)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            (
                "NaturalMatrix.concat_vertical(&NaturalMatrix)",
                &mut |(a, b)| {
                    no_out!(a.concat_vertical(&b));
                },
            ),
            (
                "NaturalMatrix.into_concat_vertical(NaturalMatrix)",
                &mut |(a, b)| {
                    no_out!(a.into_concat_vertical(b));
                },
            ),
        ],
    );
}
