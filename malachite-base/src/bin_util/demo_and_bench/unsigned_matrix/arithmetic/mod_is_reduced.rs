// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_is_reduced);
    register_bench!(runner, benchmark_unsigned_matrix_mod_is_reduced);
}

fn demo_unsigned_matrix_mod_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({a}).mod_is_reduced(&{m}) = {}", a.mod_is_reduced(&m));
    }
}

fn benchmark_unsigned_matrix_mod_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_is_reduced(&u64)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [("Malachite", &mut |(a, m)| no_out!(a.mod_is_reduced(&m)))],
    );
}
