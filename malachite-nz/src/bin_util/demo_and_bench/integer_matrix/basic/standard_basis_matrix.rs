// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::bucketers::quadruple_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_14;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_standard_basis_matrix);
    register_bench!(runner, benchmark_integer_matrix_standard_basis_matrix);
}

fn demo_integer_matrix_standard_basis_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (height, width, i, j) in unsigned_quadruple_gen_var_14().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::standard_basis_matrix({height}, {width}, {i}, {j}) = {}",
            IntegerMatrix::standard_basis_matrix(height, width, i, j)
        );
    }
}

fn benchmark_integer_matrix_standard_basis_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::standard_basis_matrix(u64, u64, u64, u64)",
        BenchmarkType::Single,
        unsigned_quadruple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("height", "width", "i", "j"),
        &mut [("Malachite", &mut |(height, width, i, j)| {
            no_out!(IntegerMatrix::standard_basis_matrix(height, width, i, j));
        })],
    );
}
