// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_36;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_identity);
    register_bench!(runner, benchmark_integer_matrix_identity);
}

fn demo_integer_matrix_identity(gm: GenMode, config: &GenConfig, limit: usize) {
    for size in unsigned_gen_var_36().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::identity({size}) = {}",
            IntegerMatrix::identity(size)
        );
    }
}

fn benchmark_integer_matrix_identity(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::identity(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |size| {
            no_out!(IntegerMatrix::identity(size));
        })],
    );
}
