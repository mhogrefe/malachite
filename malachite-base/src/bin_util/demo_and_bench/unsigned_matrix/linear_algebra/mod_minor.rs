// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::ModMinor;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_minor);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_minor_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_minor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, i, j, m) in unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_minor({i}, {j}, {m}) = {}", (&a).mod_minor(i, j, m));
    }
}

fn benchmark_unsigned_matrix_mod_minor_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_minor(u64, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_minor(u64, u64, u64)",
                &mut |(a, i, j, m)| {
                    no_out!(a.mod_minor(i, j, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_minor(u64, u64, u64)",
                &mut |(a, i, j, m)| {
                    no_out!((&a).mod_minor(i, j, m));
                },
            ),
        ],
    );
}
