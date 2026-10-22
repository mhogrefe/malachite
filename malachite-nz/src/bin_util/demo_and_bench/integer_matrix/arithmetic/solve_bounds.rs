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
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant_bound::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_solve_bounds);
    register_bench!(runner, benchmark_integer_matrix_solve_bounds_algorithms);
}

fn demo_integer_matrix_solve_bounds(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let (n, d) = a.solve_bounds(&b);
        println!("{a}.solve_bounds({b}) = ({n}, {d})");
    }
}

fn benchmark_integer_matrix_solve_bounds_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.solve_bounds(&IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(a.solve_bounds(&b));
            }),
            ("exact Hadamard bound", &mut |(a, b)| {
                no_out!(integer_matrix_solve_bounds_naive(&a, &b));
            }),
        ],
    );
}
