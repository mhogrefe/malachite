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
    register_demo!(runner, demo_unsigned_matrix_swap_rows);
    register_bench!(runner, benchmark_unsigned_matrix_swap_rows);
}

fn demo_unsigned_matrix_swap_rows(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.swap_rows(i, j);
        println!("a := {a_old}; a.swap_rows({i}, {j}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_swap_rows(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.swap_rows(u64, u64)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [("Malachite", &mut |(mut a, i, j)| {
            a.swap_rows(i, j);
        })],
    );
}
