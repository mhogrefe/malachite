// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_pivot);
    register_demo!(runner, demo_unsigned_vector_pivot_index);
    register_bench!(runner, benchmark_unsigned_vector_pivot);
    register_bench!(runner, benchmark_unsigned_vector_pivot_index);
}

fn demo_unsigned_vector_pivot(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v}.pivot() = {:?}", v.pivot());
    }
}

fn benchmark_unsigned_vector_pivot(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "UnsignedVector<u64>.pivot()",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.pivot()))],
    );
}

fn demo_unsigned_vector_pivot_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v}.pivot_index() = {:?}", v.pivot_index());
    }
}

fn benchmark_unsigned_vector_pivot_index(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.pivot_index()",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.pivot_index()))],
    );
}
