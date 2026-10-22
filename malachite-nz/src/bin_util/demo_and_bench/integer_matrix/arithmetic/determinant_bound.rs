// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant_bound::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_determinant_bound);
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_bound_algorithms
    );
}

fn demo_integer_matrix_determinant_bound(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("{a}.determinant_bound() = {}", a.determinant_bound());
    }
}

fn benchmark_integer_matrix_determinant_bound_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.determinant_bound()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.determinant_bound());
            }),
            ("exact Hadamard bound", &mut |a| {
                no_out!(integer_matrix_hadamard_bound_naive(&a, true));
            }),
        ],
    );
}
