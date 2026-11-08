// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::Minor;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_unsigned_unsigned_triple_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_minor);
    register_bench!(runner, benchmark_rational_matrix_minor_evaluation_strategy);
}

fn demo_rational_matrix_minor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, i, j) in rational_matrix_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.minor({i}, {j}) = {}", (&a).minor(i, j));
    }
}

fn benchmark_rational_matrix_minor_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.minor(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.minor(u64, u64)", &mut |(a, i, j)| {
                no_out!(a.minor(i, j));
            }),
            ("(&RationalMatrix).minor(u64, u64)", &mut |(a, i, j)| {
                no_out!((&a).minor(i, j));
            }),
        ],
    );
}
