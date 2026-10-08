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
use malachite_nz::test_util::bench::bucketers::natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_gen;
use malachite_q::rational_vector::RationalVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_from_natural_vector);
    register_bench!(runner, benchmark_rational_vector_from_natural_vector);
}

fn demo_rational_vector_from_natural_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!(
            "RationalVector::from({}) = {}",
            v.clone(),
            RationalVector::from(v)
        );
    }
}

fn benchmark_rational_vector_from_natural_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector::from(NaturalVector)",
        BenchmarkType::Single,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            let _ = RationalVector::from(v);
        })],
    );
}
