// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_53;
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_matrix::RationalMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_rectangular_identity);
    register_bench!(runner, benchmark_rational_matrix_rectangular_identity);
}

fn demo_rational_matrix_rectangular_identity(gm: GenMode, config: &GenConfig, limit: usize) {
    for (height, width) in unsigned_pair_gen_var_53().get(gm, config).take(limit) {
        println!(
            "RationalMatrix::rectangular_identity({height}, {width}) = {}",
            RationalMatrix::rectangular_identity(height, width)
        );
    }
}

fn benchmark_rational_matrix_rectangular_identity(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::rectangular_identity(u64, u64)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_53().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("height", "width"),
        &mut [("Malachite", &mut |(height, width)| {
            no_out!(RationalMatrix::rectangular_identity(height, width));
        })],
    );
}
