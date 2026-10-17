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
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_unsigned_triple_gen,
    unsigned_matrix_unsigned_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_index);
    register_demo!(runner, demo_unsigned_matrix_index_mut);
    register_demo!(runner, demo_unsigned_matrix_get);
    register_demo!(runner, demo_unsigned_matrix_get_mut);
    register_bench!(runner, benchmark_unsigned_matrix_index);
    register_bench!(runner, benchmark_unsigned_matrix_index_mut);
    register_bench!(runner, benchmark_unsigned_matrix_get);
    register_bench!(runner, benchmark_unsigned_matrix_get_mut);
}

fn demo_unsigned_matrix_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("{m}[({i}, {j})] = {}", m[(i, j)]);
    }
}

fn demo_unsigned_matrix_index_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        m[(i, j)] ^= 1;
        println!("m := {old}; m[({i}, {j})] ^= 1; m = {m}");
    }
}

fn demo_unsigned_matrix_get(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("{m}.get({i}, {j}) = {:?}", m.get(i, j));
    }
}

fn demo_unsigned_matrix_get_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i, j) in unsigned_matrix_unsigned_unsigned_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        if let Some(x) = m.get_mut(i, j) {
            *x ^= 1;
        }
        println!("m := {old}; if let Some(x) = m.get_mut({i}, {j}) {{ *x ^= 1 }}; m = {m}");
    }
}

fn benchmark_unsigned_matrix_index(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "UnsignedMatrix[(usize, usize)]",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(m, i, j)| {
            no_out!(&m[(i, j)]);
        })],
    );
}

fn benchmark_unsigned_matrix_index_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix[(usize, usize)] += u64",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, j)| {
            m[(i, j)] ^= 1;
        })],
    );
}

fn benchmark_unsigned_matrix_get(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "UnsignedMatrix.get(u64, u64)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(m, i, j)| {
            no_out!(m.get(i, j));
        })],
    );
}

fn benchmark_unsigned_matrix_get_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.get_mut(u64, u64)",
        BenchmarkType::Single,
        unsigned_matrix_unsigned_unsigned_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i, j)| {
            no_out!(m.get_mut(i, j));
        })],
    );
}
