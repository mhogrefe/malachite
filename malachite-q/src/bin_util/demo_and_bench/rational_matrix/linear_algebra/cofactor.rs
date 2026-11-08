// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::Cofactor;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_unsigned_unsigned_triple_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_cofactor);
    register_bench!(
        runner,
        benchmark_rational_matrix_cofactor_evaluation_strategy
    );
}

fn demo_rational_matrix_cofactor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, i, j) in rational_matrix_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.cofactor({i}, {j}) = {}", (&a).cofactor(i, j));
    }
}

fn benchmark_rational_matrix_cofactor_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.cofactor(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.cofactor(u64, u64)", &mut |(a, i, j)| {
                no_out!(a.cofactor(i, j));
            }),
            ("(&RationalMatrix).cofactor(u64, u64)", &mut |(a, i, j)| {
                no_out!((&a).cofactor(i, j));
            }),
        ],
    );
}
