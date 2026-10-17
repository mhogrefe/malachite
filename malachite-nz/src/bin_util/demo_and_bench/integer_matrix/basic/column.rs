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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_unsigned_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_column);
    register_bench!(runner, benchmark_integer_matrix_column);
}

fn demo_integer_matrix_column(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, j) in integer_matrix_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("{m}.column({j}) = {}", m.column(j));
    }
}

fn benchmark_integer_matrix_column(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "IntegerMatrix.column(usize)",
        BenchmarkType::Single,
        integer_matrix_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(m, j)| {
            no_out!(m.column(j));
        })],
    );
}
