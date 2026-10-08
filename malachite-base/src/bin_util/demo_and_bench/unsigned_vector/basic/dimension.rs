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

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_dimension);
    register_bench!(runner, benchmark_unsigned_vector_dimension);
}

fn demo_unsigned_vector_dimension(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v}.dimension() = {}", v.dimension());
    }
}

fn benchmark_unsigned_vector_dimension(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.dimension()",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.dimension()))],
    );
}
