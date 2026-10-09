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
use malachite_base::vector::Vector;
use malachite_q::test_util::bench::bucketers::pair_rational_vector_max_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_set_dimension);
    register_bench!(runner, benchmark_rational_vector_set_dimension);
}

fn demo_rational_vector_set_dimension(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in rational_vector_pair_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        let dimension = w.dimension();
        v.set_dimension(dimension);
        println!("v := {v_old}; v.set_dimension({dimension}); v = {v}");
    }
}

fn benchmark_rational_vector_set_dimension(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.set_dimension(u64)",
        BenchmarkType::Single,
        rational_vector_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [("Malachite", &mut |(mut v, w)| {
            v.set_dimension(w.dimension());
        })],
    );
}
