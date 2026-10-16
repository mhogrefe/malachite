// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_from_unsigned_matrix);
    register_bench!(runner, benchmark_integer_matrix_from_unsigned_matrix);
}

fn demo_integer_matrix_from_unsigned_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::from({}) = {}",
            m.clone(),
            IntegerMatrix::from(m)
        );
    }
}

fn benchmark_integer_matrix_from_unsigned_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::from(UnsignedMatrix<u64>)",
        BenchmarkType::Single,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            let _ = IntegerMatrix::from(m);
        })],
    );
}
