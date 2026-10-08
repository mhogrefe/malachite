// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_index);
    register_demo!(runner, demo_natural_vector_index_mut);
    register_bench!(runner, benchmark_natural_vector_index);
    register_bench!(runner, benchmark_natural_vector_index_mut);
}

fn demo_natural_vector_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, i) in natural_vector_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("{v}[{i}] = {}", v[i]);
    }
}

fn demo_natural_vector_index_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, i) in natural_vector_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = v.clone();
        v[i] += Natural::ONE;
        println!("v := {old}; v[{i}] += 1; v = {v}");
    }
}

fn benchmark_natural_vector_index(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "NaturalVector[usize]",
        BenchmarkType::Single,
        natural_vector_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |(v, i)| no_out!(&v[i]))],
    );
}

fn benchmark_natural_vector_index_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector[usize] += Natural",
        BenchmarkType::Single,
        natural_vector_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |(mut v, i)| v[i] += Natural::ONE)],
    );
}
