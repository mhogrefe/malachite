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
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_pivot);
    register_demo!(runner, demo_rational_vector_pivot_index);
    register_bench!(runner, benchmark_rational_vector_pivot);
    register_bench!(runner, benchmark_rational_vector_pivot_index);
}

fn demo_rational_vector_pivot(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v}.pivot() = {:?}", v.pivot());
    }
}

fn benchmark_rational_vector_pivot(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "RationalVector.pivot()",
        BenchmarkType::Single,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.pivot()))],
    );
}

fn demo_rational_vector_pivot_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v}.pivot_index() = {:?}", v.pivot_index());
    }
}

fn benchmark_rational_vector_pivot_index(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.pivot_index()",
        BenchmarkType::Single,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.pivot_index()))],
    );
}
