// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_set_row);
    register_demo!(runner, demo_unsigned_matrix_set_row_owned);
    register_bench!(runner, benchmark_unsigned_matrix_set_row);
    register_bench!(runner, benchmark_unsigned_matrix_set_row_owned);
}

fn demo_unsigned_matrix_set_row(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, v) in unsigned_matrix_unsigned_unsigned_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        m.set_row(i, &v);
        println!("m := {old}; m.set_row({i}, &{v}); m = {m}");
    }
}

fn demo_unsigned_matrix_set_row_owned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, v) in unsigned_matrix_unsigned_unsigned_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        let v_old = v.clone();
        m.set_row_owned(i, v);
        println!("m := {old}; m.set_row_owned({i}, {v_old}); m = {m}");
    }
}

fn benchmark_unsigned_matrix_set_row(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.set_row(usize, &UnsignedVector)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, v)| {
            m.set_row(i, &v);
        })],
    );
}

fn benchmark_unsigned_matrix_set_row_owned(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.set_row_owned(usize, UnsignedVector)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, v)| {
            m.set_row_owned(i, v);
        })],
    );
}
