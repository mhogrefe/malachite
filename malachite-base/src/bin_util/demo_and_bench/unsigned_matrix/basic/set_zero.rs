// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_set_zero);
    register_bench!(runner, benchmark_unsigned_matrix_set_zero);
}

fn demo_unsigned_matrix_set_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let old = m.clone();
        m.set_zero();
        println!("m := {old}; m.set_zero(); m = {m}");
    }
}

fn benchmark_unsigned_matrix_set_zero(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.set_zero()",
        BenchmarkType::Single,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |mut m| {
            m.set_zero();
        })],
    );
}
