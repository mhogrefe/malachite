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
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::bench::bucketers::pair_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_from_numerators_and_denominator);
    register_bench!(
        runner,
        benchmark_rational_vector_from_numerators_and_denominator
    );
}

fn demo_rational_vector_from_numerators_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let (ns, d) = v.to_numerators_and_denominator();
        println!(
            "from_numerators_and_denominator({ns}, {d}) = {}",
            RationalVector::from_numerators_and_denominator(&ns, &d)
        );
    }
}

fn benchmark_rational_vector_from_numerators_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "from_numerators_and_denominator(&IntegerVector, &Natural)",
        BenchmarkType::Single,
        rational_vector_gen()
            .get(gm, config)
            .map(|v| (v.clone(), v.to_numerators_and_denominator())),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |(_, (ns, d))| {
            no_out!(RationalVector::from_numerators_and_denominator(&ns, &d));
        })],
    );
}
