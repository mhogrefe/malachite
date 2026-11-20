// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::number_theory::bernoulli_number_denominator::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_bernoulli_number_denominator);
    register_bench!(
        runner,
        benchmark_natural_bernoulli_number_denominator_algorithms
    );
}

fn demo_natural_bernoulli_number_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u16>().get(gm, config).take(limit) {
        println!(
            "bernoulli_number_denominator({}) = {}",
            n,
            Natural::bernoulli_number_denominator(u64::from(n))
        );
    }
}

fn benchmark_natural_bernoulli_number_denominator_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::bernoulli_number_denominator(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(Natural::bernoulli_number_denominator(u64::from(n)));
            }),
            ("naive", &mut |n| {
                no_out!(bernoulli_number_denominator_naive(u64::from(n)));
            }),
        ],
    );
}
