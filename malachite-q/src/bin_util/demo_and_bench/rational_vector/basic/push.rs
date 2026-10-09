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
    register_demo!(runner, demo_rational_vector_push);
    register_bench!(runner, benchmark_rational_vector_push);
}

fn demo_rational_vector_push(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in rational_vector_pair_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        for x in w.elements.clone() {
            v.push(x);
        }
        println!("v := {v_old}; for x in {w} {{ v.push(x) }}; v = {v}");
    }
}

fn benchmark_rational_vector_push(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "RationalVector.push(Rational)",
        BenchmarkType::Single,
        rational_vector_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [("Malachite", &mut |(mut v, w)| {
            for x in w.elements {
                v.push(x);
            }
        })],
    );
}
