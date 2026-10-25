// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Rank;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_rank);
    register_demo!(runner, demo_natural_matrix_rank_val);
    register_bench!(runner, benchmark_natural_matrix_rank_evaluation_strategy);
}

fn demo_natural_matrix_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        println!("rank({a}) = {}", (&a).rank());
    }
}

fn demo_natural_matrix_rank_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.rank() = {}", a.rank());
    }
}

fn benchmark_natural_matrix_rank_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.rank()",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.rank()", &mut |a| {
                no_out!(a.rank());
            }),
            ("(&NaturalMatrix).rank()", &mut |a| {
                no_out!((&a).rank());
            }),
        ],
    );
}
