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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_is_zero_row);
    register_bench!(runner, benchmark_integer_matrix_is_zero_row);
}

fn demo_integer_matrix_is_zero_row(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i) in integer_matrix_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{m}.is_zero_row({i}) = {}", m.is_zero_row(i));
    }
}

fn benchmark_integer_matrix_is_zero_row(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.is_zero_row(u64)",
        BenchmarkType::Single,
        integer_matrix_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(m, i)| {
            no_out!(m.is_zero_row(i));
        })],
    );
}
