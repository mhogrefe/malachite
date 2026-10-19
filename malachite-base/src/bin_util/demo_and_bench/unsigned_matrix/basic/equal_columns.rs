// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_equal_columns);
    register_bench!(runner, benchmark_unsigned_matrix_equal_columns);
}

fn demo_unsigned_matrix_equal_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.equal_columns({i}, {j}) = {}", a.equal_columns(i, j));
    }
}

fn benchmark_unsigned_matrix_equal_columns(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.equal_columns(u64, u64)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [("Malachite", &mut |(a, i, j)| {
            no_out!(a.equal_columns(i, j));
        })],
    );
}
