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
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_reverse_rows);
    register_bench!(runner, benchmark_natural_matrix_reverse_rows);
}

fn demo_natural_matrix_reverse_rows(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in natural_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.reverse_rows();
        println!("a := {a_old}; a.reverse_rows(); a = {a}");
    }
}

fn benchmark_natural_matrix_reverse_rows(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.reverse_rows()",
        BenchmarkType::Single,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [("Malachite", &mut |mut a| {
            a.reverse_rows();
        })],
    );
}
