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
use malachite_nz::test_util::bench::bucketers::triple_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_unsigned_integer_vector_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_set_row);
    register_demo!(runner, demo_integer_matrix_set_row_owned);
    register_bench!(runner, benchmark_integer_matrix_set_row);
    register_bench!(runner, benchmark_integer_matrix_set_row_owned);
}

fn demo_integer_matrix_set_row(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, v) in integer_matrix_unsigned_integer_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        m.set_row(i, &v);
        println!("m := {old}; m.set_row({i}, &{v}); m = {m}");
    }
}

fn demo_integer_matrix_set_row_owned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, v) in integer_matrix_unsigned_integer_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        let a_old = v.clone();
        m.set_row_owned(i, v);
        println!("m := {old}; m.set_row_owned({i}, {a_old}); m = {m}");
    }
}

fn benchmark_integer_matrix_set_row(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.set_row(usize, &IntegerVector)",
        BenchmarkType::Single,
        integer_matrix_unsigned_integer_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, v)| {
            m.set_row(i, &v);
        })],
    );
}

fn benchmark_integer_matrix_set_row_owned(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.set_row_owned(usize, IntegerVector)",
        BenchmarkType::Single,
        integer_matrix_unsigned_integer_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, v)| {
            m.set_row_owned(i, v);
        })],
    );
}
