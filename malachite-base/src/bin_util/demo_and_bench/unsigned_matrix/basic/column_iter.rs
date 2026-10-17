// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::prefix_to_string;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_3;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_column_iter);
    register_demo!(runner, demo_unsigned_matrix_column_iter_mut);
    register_bench!(runner, benchmark_unsigned_matrix_column_iter);
    register_bench!(runner, benchmark_unsigned_matrix_column_iter_mut);
}

fn demo_unsigned_matrix_column_iter(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, j) in unsigned_matrix_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{m}.column_iter({j}) = {}",
            prefix_to_string(m.column_iter(j), 20)
        );
    }
}

fn demo_unsigned_matrix_column_iter_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, j) in unsigned_matrix_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        for x in m.column_iter_mut(j) {
            *x ^= 1;
        }
        println!("m := {old}; for x in m.column_iter_mut({j}) {{ *x ^= 1; }}; m = {m}");
    }
}

fn benchmark_unsigned_matrix_column_iter(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.column_iter(usize)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(m, j)| {
            for x in m.column_iter(j) {
                no_out!(x);
            }
        })],
    );
}

fn benchmark_unsigned_matrix_column_iter_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.column_iter_mut(usize)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, j)| {
            for x in m.column_iter_mut(j) {
                *x ^= 1;
            }
        })],
    );
}
