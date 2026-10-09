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
use malachite_nz::test_util::bench::bucketers::pair_natural_vector_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_extend);
    register_bench!(runner, benchmark_natural_vector_extend);
}

fn demo_natural_vector_extend(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in natural_vector_pair_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        v.extend(w.clone());
        println!("v := {v_old}; v.extend({w}); v = {v}");
    }
}

fn benchmark_natural_vector_extend(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "NaturalVector.extend(NaturalVector)",
        BenchmarkType::Single,
        natural_vector_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_vector_max_bit_bucketer("v", "w"),
        &mut [("Malachite", &mut |(mut v, w)| v.extend(w))],
    );
}
