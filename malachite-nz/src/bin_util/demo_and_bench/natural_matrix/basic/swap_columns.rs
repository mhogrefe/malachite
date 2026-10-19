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
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_swap_columns);
    register_bench!(runner, benchmark_natural_matrix_swap_columns);
}

fn demo_natural_matrix_swap_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, i, j) in natural_matrix_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.swap_columns(i, j);
        println!("a := {a_old}; a.swap_columns({i}, {j}); a = {a}");
    }
}

fn benchmark_natural_matrix_swap_columns(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.swap_columns(u64, u64)",
        BenchmarkType::Single,
        natural_matrix_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [("Malachite", &mut |(mut a, i, j)| {
            a.swap_columns(i, j);
        })],
    );
}
