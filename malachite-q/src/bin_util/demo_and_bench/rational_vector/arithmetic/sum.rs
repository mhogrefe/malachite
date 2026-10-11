// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::bench::bucketers::triple_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_sum);
    register_bench!(runner, benchmark_rational_vector_sum_algorithms);
}

fn demo_rational_vector_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, w) in rational_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "RationalVector::sum([{u}, {v}, {w}]) = {}",
            RationalVector::sum([&u, &v, &w].into_iter())
        );
    }
}

fn benchmark_rational_vector_sum_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector::sum(Iterator<Item = RationalVector>)",
        BenchmarkType::Algorithms,
        rational_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_vector_bit_bucketer("u"),
        &mut [
            ("default", &mut |(u, v, w)| {
                no_out!(RationalVector::sum([u, v, w].into_iter()));
            }),
            ("repeated addition", &mut |(u, v, w)| {
                no_out!(u + v + w);
            }),
        ],
    );
}
