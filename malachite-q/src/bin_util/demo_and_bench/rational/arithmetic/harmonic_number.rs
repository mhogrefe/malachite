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
use malachite_q::Rational;
use malachite_q::test_util::rational::arithmetic::harmonic_number::harmonic_number_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_harmonic_number);
    register_bench!(runner, benchmark_rational_harmonic_number_algorithms);
}

fn demo_rational_harmonic_number(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u16>().get(gm, config).take(limit) {
        println!(
            "harmonic_number({}) = {}",
            n,
            Rational::harmonic_number(u64::from(n))
        );
    }
}

fn benchmark_rational_harmonic_number_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::harmonic_number(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(Rational::harmonic_number(u64::from(n)));
            }),
            ("naive", &mut |n| {
                no_out!(harmonic_number_naive(u64::from(n)));
            }),
        ],
    );
}
