// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ContentChained;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_6;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_content_chained);
    register_demo!(runner, demo_unsigned_matrix_content_chained_ref);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_content_chained_evaluation_strategy
    );
}

fn demo_unsigned_matrix_content_chained(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in unsigned_matrix_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("({a_old}).content_chained({x}) = {}", a.content_chained(x));
    }
}

fn demo_unsigned_matrix_content_chained_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in unsigned_matrix_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).content_chained({x}) = {}", (&a).content_chained(x));
    }
}

fn benchmark_unsigned_matrix_content_chained_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.content_chained(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.content_chained(u64)", &mut |(a, x)| {
                no_out!(a.content_chained(x));
            }),
            ("(&UnsignedMatrix).content_chained(u64)", &mut |(a, x)| {
                no_out!((&a).content_chained(x));
            }),
        ],
    );
}
