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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_sum);
    register_bench!(runner, benchmark_natural_vector_sum_algorithms);
}

fn demo_natural_vector_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, w) in natural_vector_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "NaturalVector::sum([{u}, {v}, {w}]) = {}",
            NaturalVector::sum([&u, &v, &w].into_iter())
        );
    }
}

fn benchmark_natural_vector_sum_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::sum(Iterator<Item = NaturalVector>)",
        BenchmarkType::Algorithms,
        natural_vector_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("u"),
        &mut [
            ("default", &mut |(u, v, w)| {
                no_out!(NaturalVector::sum([u, v, w].into_iter()));
            }),
            ("repeated addition", &mut |(u, v, w)| {
                no_out!(u + v + w);
            }),
        ],
    );
}
