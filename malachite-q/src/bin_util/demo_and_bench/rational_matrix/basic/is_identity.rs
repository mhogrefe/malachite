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
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_is_identity);
    register_bench!(runner, benchmark_rational_matrix_is_identity);
}

fn demo_rational_matrix_is_identity(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        println!("{m}.is_identity() = {}", m.is_identity());
    }
}

fn benchmark_rational_matrix_is_identity(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.is_identity()",
        BenchmarkType::Single,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            no_out!(m.is_identity());
        })],
    );
}
