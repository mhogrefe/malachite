// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_from_iter);
    register_bench!(runner, benchmark_natural_vector_from_iter);
}

fn demo_natural_vector_from_iter(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::from_iter({}) = {}",
            xs.to_debug_string(),
            xs.iter().cloned().collect::<NaturalVector>()
        );
    }
}

fn benchmark_natural_vector_from_iter(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::from_iter(I)",
        BenchmarkType::Single,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(xs.into_iter().collect::<NaturalVector>());
        })],
    );
}
