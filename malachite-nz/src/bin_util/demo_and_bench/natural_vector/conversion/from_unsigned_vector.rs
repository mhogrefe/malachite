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
use malachite_nz::natural_vector::NaturalVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_from_unsigned_vector);
    register_bench!(runner, benchmark_natural_vector_from_unsigned_vector);
}

fn demo_natural_vector_from_unsigned_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::from({}) = {}",
            v.clone(),
            NaturalVector::from(v)
        );
    }
}

fn benchmark_natural_vector_from_unsigned_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::from(UnsignedVector<u64>)",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            let _ = NaturalVector::from(v);
        })],
    );
}
